use poise::serenity_prelude::{ChannelId, GuildId};
use crate::{Context, Error};

pub async fn entrar_canal_de_voz(g_id : GuildId, c_id : ChannelId, ctx : Context<'_>) -> Result<(), Error>{
    let manager = songbird::get(ctx.as_ref())
        .await
        .expect("Songbird client já inicializado")
        .clone();
    if let Ok(handle_lock) = manager.join(g_id, c_id).await {
        ctx.say("Entrei no canal").await?;

        // Adicionar eventos ao player aqui
    }
    else {
        ctx.say("Não consegui entrar no canal").await?;
    
    }
}