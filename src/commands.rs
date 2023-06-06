use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use serenity::model::channel::PermissionOverwrite;
use serenity::model::channel::PermissionOverwriteType;
use serenity::model::Permissions;

use crate::{Context, Error};

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    ).await?;
    Ok(())
}

/// Ping!
///
/// Enter `/ping` to be ponged
#[poise::command(slash_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Hm?"] _message: Option<String>,
) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

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
    let category_rooms: u64 = category_rooms.parse().expect("Failed to parse role ID");

    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::all(),
            deny: Default::default(),
            kind: PermissionOverwriteType::Member(user.id),
        },
        PermissionOverwrite {
            allow: Default::default(),
            deny: Permissions::all(),
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

    ctx.say("Room has been created!").await?;

    Ok(())
}

/// Register bot commands.
///
/// Enter `/register` to choose how to register the bot commands.
#[poise::command(slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
