mod commands;
mod eventos;

use anyhow::Context as _;
use poise::{PrefixFrameworkOptions};
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;


struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;



#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token : String = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

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
            commands: vec![commands::hello()], //
            ..Default::default()

        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token,
        GatewayIntents::non_privileged() |
        GatewayIntents::MESSAGE_CONTENT
    )
        .event_handler(eventos::Handler)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
