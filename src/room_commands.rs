use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use poise::{serenity_prelude as serenity, serenity_prelude::CacheHttp};
use serenity::model::{
	channel::{PermissionOverwrite, PermissionOverwriteType},
	Permissions,
};
use tracing::log;

use crate::types::Context;

#[poise::command(
	slash_command,
	subcommands("create", "key_create", "key_revoke", "name_update", "open", "close")
)]
pub async fn room(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

/// Create a new room for a guest.
///
/// Enter `/room create user` to create a new room for a specified user.
#[poise::command(slash_command)]
pub async fn create(
	ctx: Context<'_>,
	#[description = "User that will get a new room"] user: serenity::User,
) -> Result<()> {
	let sanitized_username = {
		let mut username = user.name.to_ascii_lowercase();
		username.retain(|character| character.is_ascii_alphanumeric());
		username
	};

	let room_name = format!("room-{}", sanitized_username);

	let role_everyone = ctx.data().discord_role_everyone;
	let category_rooms = ctx.data().discord_category_rooms;

	let permissions = vec![
		PermissionOverwrite {
			allow: Permissions::all(),
			deny: Default::default(),
			kind: PermissionOverwriteType::Member(user.id),
		},
		PermissionOverwrite {
			allow: Default::default(),
			deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
			kind: PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
		},
	];

	log::debug!("Creating channel {}", room_name);

	match ctx
		.partial_guild()
		.await
		.ok_or(anyhow!("Can only be called in a server."))?
		.create_channel(&ctx, |create_channel| {
			create_channel
				.name(&room_name)
				.kind(serenity::ChannelType::Voice)
				.nsfw(true)
				.permissions(permissions)
				.category(category_rooms)
		})
		.await
	{
		Ok(_) => {
			log::debug!("Created channel {}", room_name);
		}
		Err(e) => {
			log::debug!("Failed to create channel: {}", e);
			bail!(e)
		}
	}

	let discord_role_hotel_member = ctx.data().discord_role_hotel_member;

	log::debug!("Adding role to member {}", user.name);
	ctx.http()
		.add_member_role(
			ctx.guild_id()
				.ok_or(anyhow!("Can only be called in a server."))?
				.0,
			user.id.into(),
			discord_role_hotel_member,
			Some("You now have a room! :D"),
		)
		.await?;

	ctx.say("Room has been created!").await?;

	Ok(())
}

/// Create a new key for an existing room.
///
/// Enter `/room key_create user` to allow the specified user to read and send messages in your room.
#[poise::command(slash_command)]
pub async fn key_create(
	ctx: Context<'_>,
	#[description = "User that will get a key"] user: serenity::User,
) -> Result<()> {
	let channel = ctx.channel_id();

	let permissions = PermissionOverwrite {
		allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		deny: Default::default(),
		kind: PermissionOverwriteType::Member(user.id),
	};

	channel.create_permission(ctx, &permissions).await?;

	ctx.say("Room access for provided user has been granted!")
		.await?;

	Ok(())
}

/// Revoke a key for an existing room.
///
/// Enter `/room key_revoke user` to disallow the specified user to read and send messages in your room.
#[poise::command(slash_command)]
pub async fn key_revoke(
	ctx: Context<'_>,
	#[description = "User that will lose their key"] user: serenity::User,
) -> Result<()> {
	let channel = ctx.channel_id();

	let permissions = PermissionOverwrite {
		allow: Default::default(),
		deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		kind: PermissionOverwriteType::Member(user.id),
	};

	channel.create_permission(ctx, &permissions).await?;

	ctx.say("Room access for provided user has been revoked!")
		.await?;

	Ok(())
}

/// TODO
///
/// Enter `/room name_update user`
#[poise::command(slash_command)]
pub async fn name_update(
	_ctx: Context<'_>,
	#[description = "User that will get a new room"] _user: serenity::User,
) -> Result<()> {
	// let channel = ctx.channel_id();

	// let permissions = PermissionOverwrite {
	// 	allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
	// 	deny: Default::default(),
	// 	kind: PermissionOverwriteType::Member(user.id),
	// };

	// channel.create_permission(ctx, &permissions).await?;

	// ctx.say("Room access for provided user has been granted!")
	// 	.await?;

	Ok(())
}

/// Open a room's door, allowing everyone to view and connect.
///
/// Enter `/room open` to open your room's door.
#[poise::command(slash_command)]
pub async fn open(ctx: Context<'_>) -> Result<()> {
	let role_everyone = ctx.data().discord_role_everyone;

	let permissions = PermissionOverwrite {
		allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
		deny: Default::default(),
		kind: PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
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
		kind: PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
	};

	ctx.channel_id()
		.delete_permission(
			&ctx,
			PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
		)
		.await?;

	ctx.channel_id()
		.create_permission(&ctx, &permissions)
		.await?;

	ctx.say("Room has been closed!").await?;

	Ok(())
}
