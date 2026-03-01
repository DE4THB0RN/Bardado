mod commands;
mod dado;
mod eventos;
mod secret;
mod statuses;
mod music;

use poise::PrefixFrameworkOptions;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::prelude::TypeMapKey;
use crate::eventos::Handler;
use songbird::SerenityInit;
use reqwest::Client as HttpClient;

struct Data {
    
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

impl Data {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

async fn get_http_client(ctx: &Context) -> HttpClient {
    let data = ctx.data();
    data.get::<HttpKey>()
        .cloned()
        .expect("Guaranteed to exist in the typemap.")
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
                commands::dad0(),
                commands::iniciativa(),
                commands::limpar_iniciativa(),
                commands::listar_iniciativa(),
                commands::mudar_iniciativa(),
                commands::kanka(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data::new())
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let client = ClientBuilder::new(
        discord_token,
        intents
    )
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await;

    client.unwrap().start().await.unwrap();
}
