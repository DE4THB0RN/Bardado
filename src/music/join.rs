use poise::command;
use crate::{Context, Error};
use crate::music::uteis::entrar_canal_de_voz;

#[command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Você nem tá num canal de voz").await?;
            return Ok(());
        }
    };

    entrar_canal_de_voz(guild_id, connect_to, ctx).await
}