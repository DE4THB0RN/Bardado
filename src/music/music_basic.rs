use crate::Context;
use crate::Error;

use std::ops::Deref;
use lavalink_rs::prelude::*;

use poise::serenity_prelude as serenity;
use serenity::{model::id::ChannelId, Http};

use songbird::ConnectionInfo as SongBirdConnectionInfo;
use lavalink_rs::model::player::ConnectionInfo as LavaLinkConnectionInfo;

struct ConnectionInfoAdapter {
    pub endpoint : String,
    pub session_id : String,
    pub token : String,
}

impl From<SongBirdConnectionInfo> for ConnectionInfoAdapter{
    fn from(songird_info: SongBirdConnectionInfo) -> Self {
        Self {
            endpoint: songird_info.endpoint,
            session_id: songird_info.session_id,
            token: songird_info.token,
        }
    }
}

impl From<ConnectionInfoAdapter> for LavaLinkConnectionInfo {
    fn from(adapter : ConnectionInfoAdapter) -> Self{
        Self {
            endpoint: adapter.endpoint,
            session_id: adapter.session_id,
            token: adapter.token,
        }
    }
}

async fn _join(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    channel_id: Option<ChannelId>,
) -> Result<bool, Error> {
    let lava_client = ctx.data().lavalink.clone();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if lava_client.get_player_context(gi).is_none() {
        let connect_to = match channel_id {
            Some(x) => x,
            None => {
                let guild = ctx.guild().unwrap().deref().clone();
                let user_channel_id = guild
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id);

                match user_channel_id {
                    Some(channel) => channel,
                    None => {
                        ctx.say("Você nem tá num canal de voz bro").await?;

                        return Err("Not in a voice channel".into());
                    }
                }
            }
        };

        let handler = manager.join_gateway(guild_id, connect_to).await;

        match handler {
            Ok((connection_info, _)) => {
                let cid = ConnectionInfoAdapter::from(connection_info);
                let ci = LavaLinkConnectionInfo::from(cid);
                lava_client
                    // The turbofish here is Optional, but it helps to figure out what type to
                    // provide in `PlayerContext::data()`
                    //
                    // While a tuple is used here as an example, you are free to use a custom
                    // public structure with whatever data you wish.
                    // This custom data is also present in the Client if you wish to have the
                    // shared data be more global, rather than centralized to each player.
                    .create_player_context_with_data::<(ChannelId, std::sync::Arc<Http>)>(
                        gi,
                        ci,
                        std::sync::Arc::new((
                            ctx.channel_id(),
                            ctx.serenity_context().http.clone(),
                        )),
                    )
                    .await?;

                //ctx.say(format!("Dei join {}", connect_to.mention())).await?;

                return Ok(true);
            }
            Err(why) => {
                ctx.say(format!("FUI BARRADO: {}", why))
                    .await?;
                return Err(why.into());
            }
        }
    }

    Ok(false)
}

/// Solta o som DJ! (F Pedro DJ,meu sensei)
#[poise::command(
    slash_command,
    prefix_command,
    aliases("p"),
)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Nombre o URL"]
    #[rest]
    term: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let has_joined = _join(&ctx, guild_id, None).await?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (preciso estar em um canal)").await?;
        return Ok(());
    };

    let query = if let Some(term) = term {
        if term.starts_with("http") {
            term
        } else {
            SearchEngines::YouTube.to_query(&term)?;
            //SearchEngines::Deezer.to_query(&term)?;
            SearchEngines::SoundCloud.to_query(&term)?;
            SearchEngines::Spotify.to_query(&term)?
        }
    } else {
        if let Ok(player_data) = player.get_player().await {
            let queue = player.get_queue();

            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player.skip()?;
            } else {
                ctx.say("A fila tá vazia").await?;
            }
        }

        return Ok(());
    };

    let loaded_tracks = lava_client.load_tracks(gi, &query).await?;

    let mut playlist_info = None;

    let mut tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            ctx.say(format!("{:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        ctx.say(format!("Playlist adicionada: {}", info.name,))
            .await?;
    } else {
        let track = &tracks[0].track;

        if let Some(uri) = &track.info.uri {
            ctx.say(format!(
                "Adicionado na fila: [{} - {}](<{}>)",
                track.info.author, track.info.title, uri
            ))
                .await?;
        } else {
            ctx.say(format!(
                "Adicionado na fila: {} - {}",
                track.info.author, track.info.title
            ))
                .await?;
        }
    }

    for i in &mut tracks {
        i.track.user_data = Some(serde_json::json!({"requester_id": ctx.author().id.get()}));
    }

    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if has_joined {
        return Ok(());
    }

    if let Ok(player_data) = player.get_player().await {
        if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
            player.skip()?;
        }
    }

    Ok(())
}

/// Dá join no canal (basicamente inutil)
#[poise::command(slash_command, prefix_command)]
pub async fn venha(
    ctx: Context<'_>,
    #[description = "Me traz pro canal"]
    #[channel_types("Voice")]
    channel_id: Option<ChannelId>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    _join(&ctx, guild_id, channel_id).await?;

    Ok(())
}
/// Irei-me embora
#[poise::command(slash_command, prefix_command)]
pub async fn adeus(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let lava_client = ctx.data().lavalink.clone();

    lava_client.delete_player(gi).await?;

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }

    ctx.say("Adiós compañeros de este largo viaje").await?;

    Ok(())
}
