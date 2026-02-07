use crate::music::music_utils::_join;
use crate::Context;
use crate::Error;

use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;

/// Dá join no canal (basicamente inutil)
#[poise::command(slash_command, prefix_command)]
pub async fn venha(
    ctx: Context<'_>,
    #[description = "Me traz pro canal"]
    #[channel_types("Voice")]
    channel_id: Option<ChannelId>,
) -> Result<(), Error> {
    _join(&ctx, channel_id).await?;

    Ok(())
}
