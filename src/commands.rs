use poise::serenity_prelude::*;
use crate::{Context, Error};

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> std::result::Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}

/// Ping!
///
/// Enter `~ping` to be ponged
#[poise::command(prefix_command, slash_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Hm?"] _message: Option<String>,
) -> std::result::Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// Create a new room for a guest.
///
/// Enter `/room create user` to create a new room for a specified user.
#[poise::command(prefix_command)]
pub async fn room_create(
    ctx: Context<'_>,
    #[description = "User that will get a new room"] user: User,
) -> std::result::Result<(), Error> {
    let guild = ctx.guild().expect("Can only be called in a server.");

    let author_name = ctx.author().name.to_owned();
    let room_name = format!("room-{}", author_name.to_lowercase());

    guild.create_channel(ctx.http(), |create_channel| create_channel
        .kind(ChannelType::Voice)
        .name(room_name)
        .nsfw(true)
    ).await?;

    Ok(())
}
