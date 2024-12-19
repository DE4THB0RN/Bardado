use std::sync::Arc;
use lavalink_rs::{hook, model::events, prelude::*};
use lavalink_rs::model::events::TrackStart;
use poise::serenity_prelude::{model::id::ChannelId, Http};
use tracing::info;
use crate::statuses;

#[hook]
pub async fn raw_event(_: LavalinkClient, session_id: String, event: &serde_json::Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        info!("{:?} -> {:?}", session_id, event);
    }
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &TrackStart) {
    let player_context = client.get_player_context(event.guild_id).unwrap();
    let data = player_context
        .data::<(ChannelId, Arc<Http>)>()
        .unwrap();
    let (channel_id, http) = (&data.0, &data.1);

    repeteco(event,client).await;

    let msg = {
        let track = &event.track;

        if let Some(uri) = &track.info.uri {
            format!(
                "Agora tocando: [{} - {}](<{}>) | A pedido de <@!{}>",
                track.info.author,
                track.info.title,
                uri,
                track.user_data.clone().unwrap()["requester_id"]
            )
        } else {
            format!(
                "Agora tocando: {} - {} | A pedido de <@!{}>",
                track.info.author,
                track.info.title,
                track.user_data.clone().unwrap()["requester_id"]
            )
        }
    };

    let _ = channel_id.say(http, msg).await;
}

async fn repeteco(event:&TrackStart, lavalink_client: LavalinkClient){

    let loop_status = statuses::SHARED_LOOP.lock().unwrap().loop_track;

    if loop_status {
        if let Some(node) = lavalink_client.get_player_context(event.guild_id){
            if let Some(track) = node.get_player().await.unwrap().track {
                let queue = node.get_queue();
                queue.push_to_front(track).unwrap()
            }
        }
    }
}