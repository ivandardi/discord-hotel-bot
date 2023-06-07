#![warn(clippy::str_to_string)]

use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;

use anyhow::Context as _;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use types::{Data, Error};

mod commands;
mod room_commands;
mod types;

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    let data = Data::new(&secret_store);

    let framework = poise::Framework::builder()
        .token(secret_store.get("DISCORD_TOKEN").unwrap())
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                println!("Registering commands...");
                poise::builtins::register_in_guild(_ctx, &_framework.options().commands, serenity::GuildId(data.discord_guild)).await?;
                println!("Logged in as {}", _ready.user.name);
                Ok(data)
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
                room_commands::room_close(),
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
            serenity::GatewayIntents::all(),
        )
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
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
