#![warn(clippy::str_to_string)]

use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GuildId;

mod commands;
mod room_commands;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;

// Custom user data passed to all command functions
pub struct Data {}

type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let guild_id = dotenv!("DISCORD_GUILD");
    let guild_id: GuildId = guild_id.parse::<u64>().expect("Failed to parse guild ID").into();

    let framework = poise::Framework::builder()
        .token(dotenv!("DISCORD_TOKEN"))
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                println!("Registering commands...");
                poise::builtins::register_in_guild(_ctx, &_framework.options().commands, guild_id).await?;
                println!("Logged in as {}", _ready.user.name);
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help(),
                commands::ping(),
                commands::register(),
                room_commands::room_create(),
                room_commands::room_key_create(),
                room_commands::room_open(),
            ],
            /// The global error handler for all error cases that may occur
            on_error: |error| Box::pin(on_error(error)),
            /// This code is run before every command
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            /// This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            /// Every command invocation must pass this check to continue execution
            command_check: Some(|_ctx| {
                Box::pin(async move {
                    Ok(true)
                })
            }),
            /// Enforce command checks even for owners (enforced by default)
            /// Set to true to bypass checks, which is useful for testing
            skip_checks_for_owners: false,
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    println!("Got an event in event handler: {:?}", event.name());
                    Ok(())
                })
            },
            ..Default::default()
        })
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .build()
        .await
        .unwrap();

    framework.start().await.unwrap();
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error);
        }
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error, );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}
