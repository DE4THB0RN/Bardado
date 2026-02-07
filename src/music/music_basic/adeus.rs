use crate::Context;
use crate::Error;

/// Irei-me embora
#[poise::command(slash_command, prefix_command)]
pub async fn adeus(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }

    ctx.say("Adiós compañeros de este largo viaje").await?;

    Ok(())
}


