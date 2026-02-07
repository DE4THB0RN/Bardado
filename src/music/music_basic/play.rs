use std::sync::Arc;

use crate::music::music_utils::TrackPlayNotifier;
use crate::music::music_utils::_join;
use crate::Context;
use crate::Error;
use crate::HttpKey;

use songbird::input::Input;
use songbird::input::YoutubeDl;
use songbird::Event;
use songbird::TrackEvent;
use std::process::Command;
use tokio::task;
use tokio_util::sync::CancellationToken;

/// Solta o som DJ! (F Pedro DJ,meu sensei)
#[poise::command(slash_command, prefix_command, aliases("p"))]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Nombre o busca"]
    #[rest]
    term: Option<String>,
) -> Result<(), Error> {
    let guild_id = {
        let guild = ctx.guild().ok_or("Guild not found")?;
        guild.id
    };

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .ok_or(Error::from("Erro ao gerar http_client"))?
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client já foi inicializado")
        .clone();

    if manager.get(guild_id).is_none() {
        _join(&ctx, None).await?;
    }

    let handler_lock = manager.get(guild_id.clone()).ok_or("Handler not found")?;
    let mut handler = handler_lock.lock().await;

    let busca = {
        match term {
            Some(query) => {
                ctx.say(format!("Buscando por {query}...")).await?;
                query
            }
            None => {
                return match handler.queue().current() {
                    Some(track) => {
                        track.play()?;
                        ctx.say(format!("Voltando com a música")).await?;
                        Ok(())
                    }
                    None => {
                        ctx.say("Nenhuma música na fila").await?;
                        Err("Nenhuma música na fila".into())
                    }
                };
            }
        }
    };

    if busca.starts_with("http") && (busca.contains("playlist") || busca.contains("list=")) {
        let playlist_urls = extract_playlist_urls(&busca).await.unwrap();

        if playlist_urls.is_empty() {
            ctx.say("Não tem nenhuma música nessa playlist").await?;
            return Ok(());
        }

        let limite = playlist_urls.len() >= MAX_PLAYLIST_TRACKS;
        ctx.say(if limite {
            format!("Já tô com {MAX_PLAYLIST_TRACKS} músicas aqui pra tocar, tenha calma filhão")
        } else {
            format!("Carregando {} músicas aqui...", playlist_urls.len())
        })
        .await?;

        let handler_clone = handler_lock.clone();
        let http_client_clone = http_client.clone();
        let channel_id = ctx.channel_id();
        let serenity_http = Arc::clone(&ctx.serenity_context().http);

        let cancel_tokens = ctx.data().playlist_cancel_tokens.clone();
        {
            let tokens = cancel_tokens.read().await;
            if let Some(existing_token) = tokens.get(&guild_id) {
                existing_token.cancel();
            }
        }

        let cancel_toke = CancellationToken::new();
        {
            let mut tokens = cancel_tokens.write().await;
            tokens.insert(guild_id.clone(), cancel_toke.clone());
        }

        drop(handler);

        task::spawn(async move {
            for url in playlist_urls {
                if cancel_toke.is_cancelled() {
                    break;
                }

                let src = YoutubeDl::new(http_client_clone.clone(), url);
                let mut input: Input = src.into();

                match input.aux_metadata().await {
                    Err(_e) => {
                        continue;
                    }
                    Ok(metadata) => {
                        if cancel_toke.is_cancelled() {
                            break;
                        }

                        let mut handler = handler_clone.lock().await;
                        let track_handle: songbird::tracks::TrackHandle =
                            handler.enqueue_input(input).await;

                        let song_title = metadata
                            .title
                            .clone()
                            .unwrap_or_else(|| "Unknown Title".to_string());

                        let _ = track_handle.add_event(
                            Event::Track(TrackEvent::Play),
                            TrackPlayNotifier {
                                channel_id: channel_id,
                                http: Arc::clone(&serenity_http),
                                song_title: song_title.clone(),
                            },
                        );
                    }
                };
            }

            let mut tokens = cancel_tokens.write().await;
            tokens.remove(&guild_id);
        });
    } else {
        println!("Chegamos aqui, hora de só uma música");
        let src = if busca.starts_with("http") {
            YoutubeDl::new(http_client.clone(), busca.clone())
        } else {
            YoutubeDl::new_search(http_client.clone(), busca.clone())
        };

        println!("Conexão com YoutubeDL iniciada");

        let channel_id = ctx.channel_id();
        let serenity_http = Arc::clone(&ctx.serenity_context().http);

        let mut input: Input = src.into();

        let metadata = match input.aux_metadata().await {
            Ok(x) => x.clone(),
            Err(e) => {
                return Err(e.into());
            }
        };

        let fila = !handler.queue().is_empty();

        let track_handle = handler.enqueue_input(input).await;

        let song_title = metadata
            .title
            .clone()
            .unwrap_or_else(|| "Unknown Title".to_string());

        println!("Hora de adicionar o evento");
        track_handle.add_event(
            Event::Track(TrackEvent::Play),
            TrackPlayNotifier {
                channel_id: channel_id,
                http: Arc::clone(&serenity_http),
                song_title: song_title.clone(),
            },
        )?;

        if fila {
            ctx.say(format!("{} adicionado à fila", song_title)).await?;
        }
    }

    Ok(())
}

const MAX_PLAYLIST_TRACKS: usize = 300;

async fn extract_playlist_urls(playlist_url: &str) -> Result<Vec<String>, Error> {
    let output = Command::new("yt-dlp")
        .args([
            "--flat-playlist",
            "--print",
            "webpage_url",
            "--playlist-end",
            &MAX_PLAYLIST_TRACKS.to_string(),
            playlist_url,
        ])
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {e}"))?;

    if !output.status.success() {
        let erro = String::from_utf8_lossy(&output.stderr);
        return Err(Error::from(erro.to_string()));
    }

    let urls = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(urls)
}
