use std::time::Duration;

use crate::{statuses, Context};
use crate::Error;

use futures::future;
use futures::stream::StreamExt;
use lavalink_rs::model::{GuildId};

/// Verifica o que tem na fila
#[poise::command(slash_command, prefix_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();

    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    let queue = player.get_queue();
    let player_data = player.get_player().await?;

    let max = queue.get_count().await?.min(9);

    let queue_message = queue
        .enumerate()
        .take_while(|(idx, _)| future::ready(*idx < max))
        .map(|(idx, x)| {
            if let Some(uri) = &x.track.info.uri {
                format!(
                    "{} -> [{} - {}](<{}>) | Pedido por <@!{}>",
                    idx + 1,
                    x.track.info.author,
                    x.track.info.title,
                    uri,
                    x.track.user_data.unwrap()["requester_id"]
                )
            } else {
                format!(
                    "{} -> {} - {} | Pedido por <@!{}",
                    idx + 1,
                    x.track.info.author,
                    x.track.info.title,
                    x.track.user_data.unwrap()["requester_id"]
                )
            }
        })
        .collect::<Vec<_>>()
        .await
        .join("\n");

    let now_playing_message = if let Some(track) = player_data.track {
        let time_s = player_data.state.position / 1000 % 60;
        let time_m = player_data.state.position / 1000 / 60;
        let time = format!("{:02}:{:02}", time_m, time_s);

        if let Some(uri) = &track.info.uri {
            format!(
                "Agora tocando: [{} - {}](<{}>) | {}, a pedido de <@!{}>",
                track.info.author,
                track.info.title,
                uri,
                time,
                track.user_data.unwrap()["requester_id"]
            )
        } else {
            format!(
                "Agora vai tocar: {} - {} | {}, a pedido de <@!{}>",
                track.info.author,
                track.info.title,
                time,
                track.user_data.unwrap()["requester_id"]
            )
        }
    } else {
        "Nada tocando...apenas dor".to_string()
    };

    ctx.say(format!("{}\n\n{}", now_playing_message, queue_message))
        .await?;

    Ok(())
}

/// Pula a musica atual
#[poise::command(slash_command, prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();

    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);
    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.skip()?;
        ctx.say(format!("Skippei {}", np.info.title)).await?;
    } else {
        ctx.say("Nada pra pular").await?;
    }

    Ok(())
}

/// Pausa a música atual
#[poise::command(slash_command, prefix_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    player.set_pause(true).await?;

    ctx.say("Musica pausada").await?;

    Ok(())
}

/// Despausa a música
#[poise::command(slash_command, prefix_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    player.set_pause(false).await?;

    ctx.say("Despausei").await?;

    Ok(())
}

/// Para completamente a música
#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if let Some(np) = now_playing {
        player.stop_now().await?;
        ctx.say(format!("Dei cabo em {}", np.info.title)).await?;
    } else {
        ctx.say("Não tem nada aqui").await?;
    }

    Ok(())
}

/// Pula pra uma parte específica da música (em segundos plz)
#[poise::command(slash_command, prefix_command)]
pub async fn seek(
    ctx: Context<'_>,
    #[description = "Escolha pra onde pular(em segundos)"] time: u64,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    let now_playing = player.get_player().await?.track;

    if now_playing.is_some() {
        player.set_position(Duration::from_secs(time)).await?;
        ctx.say(format!("Pulei pra {}s", time)).await?;
    } else {
        ctx.say("Não tem nada aqui tocando").await?;
    }

    Ok(())
}

/// Remove uma música específica
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Index pra remover"] index: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    player.get_queue().remove(index)?;

    ctx.say("Removido com sucesso").await?;

    Ok(())
}

/// Limpa a fila completamente
#[poise::command(slash_command, prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    player.get_queue().clear()?;

    ctx.say("Fila tá vazia agora").await?;

    Ok(())
}

/// Shambles! Troca a posição de duas músicas
#[poise::command(slash_command, prefix_command)]
pub async fn swap(
    ctx: Context<'_>,
    #[description = "Primeiro index pra trocar"] index1: usize,
    #[description = "Segundo index pra trocar"] index2: usize,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let gid : u64 = u64::from(guild_id);

    let gi : GuildId = GuildId::from(gid);

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };

    let queue = player.get_queue();
    let queue_len = queue.get_count().await?;

    if index1 > queue_len || index2 > queue_len {
        ctx.say(format!("Olha só o engraçadinho, máximo da fila é {}", queue_len))
            .await?;
        return Ok(());
    } else if index1 == index2 {
        ctx.say("Trocar 2 iguais você tá chapando").await?;
        return Ok(());
    }

    let track1 = queue.get_track(index1 - 1).await?.unwrap();
    let track2 = queue.get_track(index2 - 1).await?.unwrap();

    queue.swap(index1 - 1, track2)?;
    queue.swap(index2 - 1, track1)?;

    ctx.say("Trocado com sucesso").await?;

    Ok(())
}

/// E de novo e de novo e de novo e de novo...
#[poise::command(
    prefix_command,
    slash_command,
    rename = "loop",
)]
pub async fn repete(ctx: Context<'_>, ) -> Result<(),Error> {
    let guild_id = ctx.guild_id().unwrap();
    let gid: u64 = u64::from(guild_id);
    let gi: GuildId = GuildId::from(gid);

    let lava_client = ctx.data().lavalink.clone();


    let Some(player) = lava_client.get_player_context(gi) else {
        ctx.say("Pelo menos me chama pra sair primeiro (tenho que estar em um canal)").await?;
        return Ok(());
    };


    let now_playing = player.get_player().await?.track;

    let np = now_playing.clone();

    let queuer = player.get_queue();

    if np.is_none() {
        ctx.say("Não tá tocando nada,chapou bro").await?;
        return Ok(());
    }
    let looper;
    {
        let mut looping_state = statuses::SHARED_LOOP.lock().unwrap();
        looping_state.loop_track = !looping_state.loop_track;
        looper = looping_state.loop_track.clone();
    }
    if looper {
        queuer.push_to_front(now_playing.unwrap()).unwrap();
        {
            ctx.say("Loop ativado!").await?;
        }

    }
    else{
        queuer.remove(0).unwrap();
        ctx.say("Loop desativado!").await?;
    }

    Ok(())
}