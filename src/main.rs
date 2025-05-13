mod commands;
mod eventos;
mod music;
mod statuses;
mod dado;

use anyhow::Context as _;
use lavalink_rs::client::LavalinkClient;
use lavalink_rs::model::events;
use lavalink_rs::node::NodeBuilder;
use lavalink_rs::prelude::NodeDistributionStrategy;
use poise::PrefixFrameworkOptions;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use songbird::SerenityInit;

struct Data {
    pub lavalink : LavalinkClient,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    std::env::set_var("RUST_LOG", "info,lavalink_rs=trace");
    // Get the discord token set in `Secrets.toml`
    let discord_token : String = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let lavalink_password : String = secret_store
        .get("LAVALINK_PASSWORD")
        .context("'LAVALINK_PASSWORD' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: PrefixFrameworkOptions{
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
                music::music_basic::play(),
                music::music_basic::venha(),
                music::music_basic::adeus(),
                music::music_advanced::queue(),
                music::music_advanced::skip(),
                music::music_advanced::pause(),
                music::music_advanced::resume(),
                music::music_advanced::stop(),
                music::music_advanced::seek(),
                music::music_advanced::clear(),
                music::music_advanced::remove(),
                music::music_advanced::swap(),
                music::music_advanced::repete(), //Vamo lá,aparece aí
                commands::dad0(),
                commands::iniciativa(),
                commands::limpar_iniciativa(),
                commands::listar_iniciativa(),
                commands::mudar_iniciativa(),
            ], //
            ..Default::default()

        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let events = events::Events {
                    raw: Some(music::music_events::raw_event),
                    ready: Some(music::music_events::ready_event),
                    track_start: Some(music::music_events::track_start),
                    ..Default::default()
                };

                let usrid : u64 = ctx.cache.current_user().id.into();

                let node_local = NodeBuilder {
                    hostname: "lavalink.jirayu.net:13592".to_string(),
                    is_ssl: false,
                    events: events::Events::default(),
                    password: lavalink_password,
                    user_id: usrid.into(),
                    session_id: None,
                };

                let client = LavalinkClient::new(
                    events,
                    vec![node_local],
                    NodeDistributionStrategy::round_robin(),
                ).await;


                Ok(Data { lavalink: client })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token,
        GatewayIntents::non_privileged() |
        GatewayIntents::MESSAGE_CONTENT
    )
        .register_songbird()
        .event_handler(eventos::Handler)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
