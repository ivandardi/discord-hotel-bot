use crate::helpers::get_unix_timestamp_now;
use anyhow::Result;
use poise::serenity_prelude::Mentionable;

use crate::types::Context;

/// Sends an alert to the responsible authorities.
///
/// The command `/alert` will create a generic alert to the authorities indicating where the command
/// was run.
///
/// You can add more context to the command with its parameters.
#[poise::command(slash_command)]
pub async fn alert(ctx: Context<'_>) -> Result<()> {
	// let cache = ctx.cache().ok_or(anyhow!("Couldn't retrieve the cache."))?;

	let channel_name = ctx.channel_id().mention();

	let time = get_unix_timestamp_now()?;

	let _sent_message = ctx
		.data()
		.discord_channel_alerts
		.send_message(&ctx, |create_message| {
			create_message.content(format!(
				"Alert was sent from channel {channel_name} <t:{time}:R>.\
				Please react to this message when you read it."
			))
		})
		.await?;

	Ok(())
}
