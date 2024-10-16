use super::{ani_list, ani_schedule};
use crate::{Context, Error};

/// A command with two subcommands: `list` and `register`
///
/// Running this function directly, without any subcommand, is only supported in prefix commands.
/// Discord doesn't permit invoking the root command of a slash command if it has subcommands.
#[poise::command(
    prefix_command,
    slash_command,
    rename = "ani",
    subcommands("list", "schedule")
)]
pub async fn root(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("DEBUG: ani main command").await?;
    Ok(())
}

// Add these two functions to re-export the subcommands with the desired names
/// List subcommand
#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ani_list::list(ctx).await
}

/// Register subcommand
#[poise::command(slash_command, prefix_command)]
pub async fn schedule(ctx: Context<'_>) -> Result<(), Error> {
    ani_schedule::schedule(ctx).await
}
