#![warn(clippy::str_to_string)]

use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;

mod commands;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;

// Custom user data passed to all command functions
pub struct Data {}

type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let framework = poise::Framework::builder()
        .token(dotenv!("DISCORD_TOKEN"))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Registering commands...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Logged in as {}", _ready.user.name);
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help(),
                commands::ping(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
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
