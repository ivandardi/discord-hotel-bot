#![warn(clippy::str_to_string)]

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};

use anyhow::Error;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use tracing::log;
use types::Data;

mod commands;
mod helpers;
mod types;

#[shuttle_runtime::main]
async fn poise(
	#[shuttle_secrets::Secrets] secret_store: SecretStore,
	#[shuttle_shared_db::Postgres] database: PgPool,
) -> ShuttlePoise<Data, Error> {
	let framework = poise::Framework::builder()
		.token(secret_store.get("DISCORD_TOKEN").unwrap())
		.setup(move |_ctx, _ready, _framework| {
			Box::pin(async move {
				let data = Data::new(&secret_store, database);

				log::info!("discord-hotel-bot logged in as {}", _ready.user.name);

				log::debug!("Registering commands...");
				poise::builtins::register_in_guild(
					_ctx,
					&_framework.options().commands,
					data.discord_guild,
				)
				.await?;

				log::debug!("Setting up database...");
				data.database
					.execute(
						"CREATE TABLE IF NOT EXISTS user_room_ownership (
						user_id BIGINT,
						channel_id BIGINT,
						PRIMARY KEY (user_id)
					);",
					)
					.await?;

				Ok(data)
			})
		})
		.options(poise::FrameworkOptions {
			prefix_options: PrefixFrameworkOptions {
				mention_as_prefix: true,
				..Default::default()
			},
			commands: vec![
				commands::help(),
				commands::ping(),
				commands::register(),
				commands::shutdown(),
				commands::room::room(),
				commands::moderation::alert(),
				commands::uptime::uptime(),
			],
			/// The global error handler for all error cases that may occur
			on_error: |error| Box::pin(on_error(error)),
			/// This code is run before every command
			pre_command: |ctx| {
				Box::pin(async move {
					log::debug!("Executing command {}...", ctx.command().qualified_name);
				})
			},
			/// This code is run after a command if it was successful (returned Ok)
			post_command: |ctx| {
				Box::pin(async move {
					log::debug!("Executed command {}!", ctx.command().qualified_name);
				})
			},
			/// Every command invocation must pass this check to continue execution
			command_check: Some(|_ctx| Box::pin(async move { Ok(true) })),
			/// Enforce command checks even for owners (enforced by default)
			/// Set to true to bypass checks, which is useful for testing
			skip_checks_for_owners: false,
			event_handler: |_ctx, event, _framework, _data| {
				Box::pin(async move {
					log::debug!("Got an event in event handler: {:?}", event.name());
					Ok(())
				})
			},
			..Default::default()
		})
		.intents(serenity::GatewayIntents::all())
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
			log::error!("Error in command `{}`: {:?}", ctx.command().name, error);
		}
		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				log::error!("Error while handling error: {}", e)
			}
		}
	}
}
