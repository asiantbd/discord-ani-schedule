use crate::{Context, Error};

/// Subcommand: schedule
/// <ani> schedule
pub async fn schedule(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Just some dummy command").await?;
    Ok(())
}
