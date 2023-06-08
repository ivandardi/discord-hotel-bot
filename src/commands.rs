use crate::types::Context;
use anyhow::Result;

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
#[poise::command(slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}
