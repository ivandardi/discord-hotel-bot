use crate::types::Context;
use anyhow::Result;

pub mod moderation;
pub mod room;
pub mod uptime;

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Specific command to show help about"]
	#[autocomplete = "poise::builtins::autocomplete_command"]
	command: Option<String>,
) -> Result<()> {
	poise::builtins::help(
		ctx,
		command.as_deref(),
		poise::builtins::HelpConfiguration::default(),
	)
	.await?;
	Ok(())
}

/// Ping!
///
/// Enter `/ping` to be ponged
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>, #[description = "Hm?"] _message: Option<String>) -> Result<()> {
	ctx.say("Pong!").await?;
	Ok(())
}

/// Register bot commands.
///
/// Enter `/register` to choose how to register the bot commands.
#[poise::command(slash_command, prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}

/// Shuts down the bot.
///
/// Enter `/shutdown` to shutdown the bot.
/// Enter `/shutdown unregister` to also unregister commands.
#[poise::command(slash_command)]
pub async fn shutdown(ctx: Context<'_>, #[flag] unregister: bool) -> Result<()> {
	if unregister {
		ctx.say(":gear: Unregistering guild commands...").await?;
		ctx.guild_id()
			.unwrap()
			.set_application_commands(ctx, |b| b)
			.await?;
	}
	// TODO: Add logout call so that the bot goes offline
	std::process::exit(0);
}
