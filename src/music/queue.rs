use poise::command;
use crate::{get_http_client, Context, Error};
use crate::music::uteis::entrar_canal_de_voz;

#[command(slash_command, prefix_command)]
pub async fn enfileirar(
    ctx: Context<'_>,
    #[description = "Uma url ou nome da música"] busca: String,
) -> Result<(), Error> {

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

    entrar_canal_de_voz(guild_id.clone(), connect_to, ctx).await?;

    let buscar = busca.starts_with("http");

    let http_client = get_http_client(&ctx).await;

    let mut manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird voz ativada")
        .clone();

    
    Ok(())
}