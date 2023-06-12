use crate::types::Context;
use anyhow::Result;

/// Tells for how long the bot has been up for.
#[poise::command(slash_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<()> {
	let time = ctx.data().bot_startup_timestamp;
	ctx.say(format!(
		"The bot has been running since <t:{time}:F> (<t:{time}:R>)"
	))
	.await?;
	Ok(())
}
