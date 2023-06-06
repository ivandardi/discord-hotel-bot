use dotenv_codegen::dotenv;
use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::Permissions;

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

    let room_name = format!("room-{}", user.name.to_lowercase());

    let role_everyone = dotenv!("DISCORD_ROLE_EVERYONE");
    let role_everyone: u64 = role_everyone.parse().expect("Failed to parse role ID");

    let category_rooms = dotenv!("DISCORD_CATEGORY_ROOMS");
    let category_rooms: u64 = category_rooms.parse().expect("Failed to parse category ID");

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

    guild.create_channel(ctx, |create_channel| create_channel
        .name(room_name)
        .kind(serenity::ChannelType::Voice)
        .nsfw(true)
        .permissions(permissions)
        .category(category_rooms),
    ).await?;

    let discord_role_hotel_member = dotenv!("DISCORD_ROLE_HOTEL_MEMBER");
    let discord_role_hotel_member: u64 = discord_role_hotel_member.parse().expect("Failed to parse discord_role_hotel_member ID");

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
    let role_everyone = dotenv!("DISCORD_ROLE_EVERYONE");
    let role_everyone: u64 = role_everyone.parse().expect("Failed to parse role ID");

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
    let role_everyone = dotenv!("DISCORD_ROLE_EVERYONE");
    let role_everyone: u64 = role_everyone.parse().expect("Failed to parse role ID");

    let permissions = PermissionOverwrite {
        allow: Default::default(),
        deny: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
        kind: PermissionOverwriteType::Role(serenity::RoleId(role_everyone)),
    };

    ctx.channel_id().create_permission(&ctx, &permissions).await?;

    ctx.say("Room has been closed!").await?;

    Ok(())
}
