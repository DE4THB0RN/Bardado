use crate::Context;
use crate::Error;
use ::serenity::all::Http;
use ::serenity::async_trait;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use songbird::Event;
use songbird::EventContext;
use songbird::EventHandler;
use std::ops::Deref;
use std::sync::Arc;

pub async fn _join(ctx: &Context<'_>, channel_id: Option<ChannelId>) -> Result<bool, Error> {
    let guild_id = ctx.guild_id().unwrap();
    let user_id = ctx.author().id;

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    let connect_to = match channel_id {
        Some(x) => x,
        None => {
            let guild = ctx.guild().unwrap().deref().clone();
            let user_channel = guild
                .voice_states
                .get(&user_id)
                .and_then(|voice_state| voice_state.channel_id);

            match user_channel {
                Some(channel) => channel,
                None => {
                    ctx.say("Você nem tá num canal de voz cara").await?;
                    return Err("Usuário não está em nenhum canal de voz".into());
                }
            }
        }
    };

    let handler = manager.join_gateway(guild_id, connect_to).await;
    match handler {
        Ok(_) => {
            return Ok(true);
        }
        Err(e) => {
            ctx.say(format!("Não consegui entrar no canal de voz: {e}"))
                .await?;
            return Err(e.into());
        }
    }
}

pub struct TrackPlayNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>,
    pub song_title: String,
}

#[async_trait]
impl EventHandler for TrackPlayNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let _ = self
            .channel_id
            .say(
                &self.http,
                format!("Começando a reproduzir: **{}**", self.song_title),
            )
            .await;

        None
    }
}
