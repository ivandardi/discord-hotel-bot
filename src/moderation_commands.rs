use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use poise::{serenity_prelude as serenity, serenity_prelude::CacheHttp};
use serenity::model::{
	channel::{PermissionOverwrite, PermissionOverwriteType},
	Permissions,
};
use tracing::log;

use crate::types::Context;

/// Sends an alert to the responsible authorities.
///
/// This is both a group and a command.
///
/// The command `/alert` will create a generic alert to the authorities indicating where the command
/// was run.
///
/// You can add more context to the command with its parameters.
#[poise::command(
	slash_command,
)]
pub async fn alert(ctx: Context<'_>) -> Result<()> {
	ctx.data().
}
