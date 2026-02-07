mod commands;
mod dado;
mod eventos;
mod music;
mod secret;
mod statuses;

use std::collections::HashMap;
use std::sync::Arc;

use poise::serenity_prelude::{ClientBuilder, GatewayIntents, GuildId};
use poise::PrefixFrameworkOptions;
use reqwest::Client;
use serenity::prelude::TypeMapKey;
use songbird::SerenityInit;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

struct Data {
    pub playlist_cancel_tokens: Arc<RwLock<HashMap<GuildId, CancellationToken>>>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = Client;
}

impl Data {
    pub fn new() -> Self {
        Self {
            playlist_cancel_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() {
    let discord_token: String = secret::get_token();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                additional_prefixes: vec![],
                dynamic_prefix: None,
                stripped_dynamic_prefix: None,
                mention_as_prefix: true,
                edit_tracker: None,
                execute_untracked_edits: false,
                ignore_edits_if_not_yet_responded: false,
                execute_self_messages: false,
                ignore_bots: true,
                ignore_thread_creation: false,
                case_insensitive_commands: true,
                __non_exhaustive: (),
            },
            commands: vec![
                music::music_basic::play::play(),
                music::music_basic::venha::venha(),
                music::music_basic::adeus::adeus(),
                // music::music_advanced::queue(),
                // music::music_advanced::skip(),
                // music::music_advanced::pause(),
                // music::music_advanced::resume(),
                // music::music_advanced::stop(),
                // music::music_advanced::seek(),
                // music::music_advanced::clear(),
                // music::music_advanced::remove(),
                // music::music_advanced::swap(),
                // music::music_advanced::repete(), //Vamo lá,aparece aí
                commands::dad0(),
                commands::iniciativa(),
                commands::limpar_iniciativa(),
                commands::listar_iniciativa(),
                commands::mudar_iniciativa(),
                commands::kanka(),
            ], //
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data::new())
            })
        })
        .build();

    let client = ClientBuilder::new(
        discord_token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .register_songbird()
    .event_handler(eventos::Handler)
    .framework(framework)
    .type_map_insert::<HttpKey>(Client::new())
    .await;

    client.unwrap().start().await.unwrap();
}
