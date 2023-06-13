use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use poise::serenity_prelude::{ChannelId, GuildChannel, Mentionable, RoleId, User};
use poise::{serenity_prelude as serenity, serenity_prelude::CacheHttp};
use serenity::model::{
	channel::{PermissionOverwrite, PermissionOverwriteType},
	Permissions,
};
use sqlx::Row;
use std::mem;
use tracing::log;

use crate::types::Context;

#[poise::command(
	slash_command,
	subcommands("create", "key_create", "key_revoke", "name_reset", "open", "close")
)]
pub async fn room(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

/// Create a new room for a guest.
///
/// Enter `/room create <user>` to create a new room for a specified user.
///
/// This is a complicated command. It has the following responsibilities:
/// 2. Add the appropriate role to the user
/// 3. Create a new voice channel
/// 1. Create an entry in the database with the created channel ID
///
/// If any of these fail, they all need to be rolled back.
#[poise::command(slash_command)]
pub async fn create(
	ctx: Context<'_>,
	#[description = "User who will get a room."] user: User,
) -> Result<()> {
	async fn create_voice_channel(ctx: &Context<'_>, user: &User) -> Result<GuildChannel> {
		let permissions = vec![
			PermissionOverwrite {
				allow: Permissions::all(),
				deny: Default::default(),
				kind: PermissionOverwriteType::Member(user.id),
			},
			PermissionOverwrite {
				allow: Default::default(),
				deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
				kind: PermissionOverwriteType::Role(ctx.data().discord_role_everyone),
			},
		];

		let room_name = generate_room_name(user);
		log::debug!("Creating channel {}", room_name);

		ctx.partial_guild()
			.await
			.ok_or(anyhow!("Can only be called in a server."))?
			.create_channel(&ctx, |channel| {
				channel
					.name(&room_name)
					.kind(serenity::ChannelType::Voice)
					.nsfw(true)
					.permissions(permissions)
					.category(ctx.data().discord_category_rooms)
			})
			.await
			.map_err(|e| anyhow!("Failed to create channel: {e}"))
	}

	let channel = create_voice_channel(&ctx, &user).await?;

	let user_id = user.id.0;
	let channel_id = channel.id.0;

	let query_results =
		sqlx::query("INSERT INTO user_room_ownership (user_id, channel_id) VALUES ($1, $2)")
			.bind::<i64>(unsafe { mem::transmute(user_id) })
			.bind::<i64>(unsafe { mem::transmute(channel_id) })
			.execute(&ctx.data().database)
			.await;

	match query_results {
		Ok(results) => {
			let values_were_inserted = results.rows_affected() == 1;
			if !values_were_inserted {
				channel.delete(&ctx).await?;
				bail!("Failed to execute query when creating room.");
			}
		}
		Err(error) => {
			channel.delete(&ctx).await?;
			bail!(error)
		}
	}

	let role = &ctx.data().discord_role_hotel_member;
	if let Err(error) = assign_role_to_member(&ctx, role, &user).await {
		channel.delete(&ctx).await?;
		bail!(error);
	}

	ctx.say(format!("Room has been created! {}", channel.mention()))
		.await?;

	Ok(())
}

/// Create a new key for an existing room.
///
/// Enter `/room key_create <user>` to allow the specified user to read and send messages in your
/// room.
#[poise::command(slash_command)]
pub async fn key_create(
	ctx: Context<'_>,
	#[description = "User that will get a key"] user: User,
) -> Result<()> {
	let channel_id = fetch_guest_room(&ctx).await?;

	let permission_overwrite = PermissionOverwrite {
		allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		deny: Default::default(),
		kind: PermissionOverwriteType::Member(user.id),
	};

	channel_id
		.create_permission(&ctx, &permission_overwrite)
		.await?;

	ctx.say("Room access for the provided user has been granted!")
		.await?;

	Ok(())
}

/// Revoke a key for an existing room.
///
/// Enter `/room key_revoke <user>` to disallow the specified user to read and send messages in your
/// room.
#[poise::command(slash_command)]
pub async fn key_revoke(
	ctx: Context<'_>,
	#[description = "User that will lose their key"] user: User,
) -> Result<()> {
	let channel_id = fetch_guest_room(&ctx).await?;

	let permission_overwrite = PermissionOverwrite {
		allow: Default::default(),
		deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		kind: PermissionOverwriteType::Member(user.id),
	};

	channel_id
		.create_permission(&ctx, &permission_overwrite)
		.await?;

	ctx.say("Room access for the provided user has been revoked!")
		.await?;

	Ok(())
}

/// Resets the name of the room to the canonical one as defined by `generate_room_name()`.
///
/// Enter `/room name_reset <user>`
#[poise::command(slash_command)]
pub async fn name_reset(
	ctx: Context<'_>,
	#[description = "User whose room's name will be reset"] user: User,
) -> Result<()> {
	let expected_room_name = generate_room_name(&user);

	let channel_id = fetch_guest_room(&ctx).await?;

	let is_name_correct = expected_room_name
		== channel_id
			.name(&ctx)
			.await
			.unwrap_or_else(|| Default::default());

	if !is_name_correct {
		channel_id
			.edit(&ctx, |c| c.name(expected_room_name))
			.await?;
		ctx.say(format!(
			"Room name for channel {} has been reset.",
			channel_id.mention()
		))
		.await?;
	}

	Ok(())
}

/// Open a room's door, allowing everyone to view and connect.
///
/// Enter `/room open` to open your room's door.
#[poise::command(slash_command)]
pub async fn open(ctx: Context<'_>) -> Result<()> {
	let permissions = PermissionOverwrite {
		allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		deny: Default::default(),
		kind: PermissionOverwriteType::Role(ctx.data().discord_role_everyone),
	};

	ctx.channel_id()
		.create_permission(&ctx, &permissions)
		.await?;

	ctx.say("Room has been opened!").await?;

	Ok(())
}

/// Close a room's door, denying everyone from viewing and connecting.
///
/// Enter `/room close` to close your room's door.
#[poise::command(slash_command)]
pub async fn close(ctx: Context<'_>) -> Result<()> {
	let role_everyone = ctx.data().discord_role_everyone;

	let permissions = PermissionOverwrite {
		allow: Default::default(),
		deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		kind: PermissionOverwriteType::Role(role_everyone),
	};

	ctx.channel_id()
		.delete_permission(&ctx, PermissionOverwriteType::Role(role_everyone))
		.await?;

	ctx.channel_id()
		.create_permission(&ctx, &permissions)
		.await?;

	ctx.say("Room has been closed!").await?;

	Ok(())
}

/// Internal helper functions

async fn fetch_guest_room(ctx: &Context<'_>) -> Result<ChannelId, anyhow::Error> {
	let author_id_as_i64: i64 = unsafe { mem::transmute(ctx.author().id.0) };

	sqlx::query("SELECT channel_id FROM user_room_ownership WHERE user_id = $1")
		.bind(author_id_as_i64)
		.fetch_optional(&ctx.data().database)
		.await
		.map_err(|e| {
			anyhow!(
				"Couldn't fetch room for user {}: {}",
				ctx.author().mention(),
				e
			)
		})?
		.ok_or_else(|| anyhow!("User {} does not have a room.", ctx.author().mention()))
		.map(|row| {
			let row_id: i64 = row.get(0);
			ChannelId(unsafe { mem::transmute(row_id) })
		})
}

async fn assign_role_to_member(ctx: &Context<'_>, role: &RoleId, user: &User) -> Result<()> {
	log::debug!("Assigning role {} to member {}", role, user);
	ctx.http()
		.add_member_role(
			ctx.guild_id()
				.ok_or(anyhow!("Can only be called in a server."))?
				.0,
			user.id.into(),
			(*role).into(),
			Some("Automatically assigned role to member."),
		)
		.await?;
	Ok(())
}

/// Generates a Discord voice channel name for a given user.
/// For now only ascii alphanumeric characters are allowed.
fn generate_room_name(user: &User) -> String {
	let mut username = user.name.to_ascii_lowercase();
	username.retain(|character| character.is_ascii_alphanumeric());
	format!("room-{}", username)
}
