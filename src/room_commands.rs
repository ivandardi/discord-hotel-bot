use anyhow::Context as _;
use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CacheHttp;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::Permissions;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use tracing::log;

use crate::types::{Context, Error};

/// Create a new room for a guest.
///
/// Enter `/room create user` to create a new room for a specified user.
#[poise::command(slash_command)]
pub async fn room_create(
    ctx: Context<'_>,
    #[description = "User that will get a new room"] user: serenity::User,
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Can only be called in a server.");

    let sanitized_username = {
        let mut username = user.name.to_ascii_lowercase();
        username.retain(|character| !character.is_ascii_whitespace());
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

    match guild.create_channel(ctx, |create_channel| create_channel
        .name(&room_name)
        .kind(serenity::ChannelType::Voice)
        .nsfw(true)
        .permissions(permissions)
        .category(category_rooms),
    ).await {
        Ok(_) => {
            log::debug!("Created channel {}", room_name);
        }
        Err(e) => {
            log::debug!("Failed to create channel: {}", e);
            return Err(e.into());
        }
    }

    let discord_role_hotel_member = ctx.data().discord_role_hotel_member;

    log::debug!("Adding role to member {}", user.name);
    ctx.http().add_member_role(
        guild.id.into(),
        user.id.into(),
        discord_role_hotel_member,
        Some("You now have a room! :D")
    ).await?;

    ctx.say("Room has been created!").await?;

    Ok(())
}

/// Create a new key for an existing room.
///
/// Enter `/room_key_create user` to allow the specified user to read and send messages in your room.
#[poise::command(slash_command)]
pub async fn room_key_create(
    ctx: Context<'_>,
    #[description = "User that will get a new room"] user: serenity::User,
) -> Result<(), Error> {
    let channel = ctx.channel_id();

    let permissions = PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
        deny: Default::default(),
        kind: PermissionOverwriteType::Member(user.id),
    };

    channel.create_permission(ctx, &permissions).await?;

    ctx.say("Room access for provided user has been granted!").await?;

    Ok(())
}

/// Create a new key for an existing room.
///
/// Enter `/room_key_create user` to allow the specified user to read and send messages in your room.
#[poise::command(slash_command)]
pub async fn room_name_update(
    ctx: Context<'_>,
    #[description = "User that will get a new room"] user: serenity::User,
) -> Result<(), Error> {
    let channel = ctx.channel_id();

    let permissions = PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
        deny: Default::default(),
        kind: PermissionOverwriteType::Member(user.id),
    };

    channel.create_permission(ctx, &permissions).await?;

    ctx.say("Room access for provided user has been granted!").await?;

    Ok(())
}

/// Open a room's door, allowing everyone to view and connect.
///
/// Enter `/room_open` to open your room's door.
#[poise::command(slash_command)]
pub async fn room_open(ctx: Context<'_>) -> Result<(), Error> {
    let role_everyone = ctx.data().discord_role_everyone;

    let permissions = PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
        deny: Default::default(),
        kind: PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
    };

    ctx.channel_id().create_permission(&ctx, &permissions).await?;

    ctx.say("Room has been opened!").await?;

    Ok(())
}

/// Close a room's door, denying everyone from viewing and connecting.
///
/// Enter `/room_close` to close your room's door.
#[poise::command(slash_command)]
pub async fn room_close(ctx: Context<'_>) -> Result<(), Error> {
    let role_everyone = ctx.data().discord_role_everyone;

    let permissions = PermissionOverwrite {
        allow: Default::default(),
        deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
        kind: PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
    };

    ctx.channel_id().delete_permission(&ctx, PermissionOverwriteType::Role(serenity::RoleId(role_everyone))).await?;

    ctx.channel_id().create_permission(&ctx, &permissions).await?;

    ctx.say("Room has been closed!").await?;

    Ok(())
}
