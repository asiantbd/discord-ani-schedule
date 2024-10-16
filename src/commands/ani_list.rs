use crate::provider::anilist;
use crate::{Context, Error};

/// Subcommand: list
/// <ani> list
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let dummy = anilist::fetch_anime_dummy().await?;
    ctx.say(format!("Dummy List API Call: \n```json\n{}\n```", dummy))
        .await?;
    Ok(())
}
