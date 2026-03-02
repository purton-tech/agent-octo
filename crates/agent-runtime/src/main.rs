use std::sync::Arc;
use std::time::Duration;

use agent_runtime::{build_agent, build_system_prompt};
use agent_runtime::config::Config;
use db::clorinde::queries::channels::{
    claim_next_channel_message, insert_channel_message, list_conversation_messages,
    update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use rig::client::ProviderClient;
use rig::completion::{Chat, Message as RigMessage};
use rig::providers::openai::Client;
use serde_json::json;
use supabase_client_realtime::{
    PostgresChangesEvent, PostgresChangesFilter, RealtimeClient,
};
use tokio::sync::Notify;
use tool_runtime::openapi_actions::OpenApiRegistry;
use tracing::{info, warn};

const SYSTEM_PROMPT: &str = include_str!("../../octo/SYSTEM_PROMPT.md");
const MAX_HISTORY_MESSAGES: i64 = 20;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let config = Config::new();
    let pool = db::create_pool(&config.application_url);
    let inbound_notify = Arc::new(Notify::new());

    let plugin_dir = format!("{}/../octo/plugins", env!("CARGO_MANIFEST_DIR"));
    let openapi_specs = OpenApiRegistry::load_specs_from_dir(&plugin_dir)?;
    let openapi_actions = Arc::new(OpenApiRegistry::from_specs(&openapi_specs));
    let system_prompt = build_system_prompt(SYSTEM_PROMPT, &openapi_actions);
    let openai_client = Client::from_env();
    let agent = build_agent(openai_client, system_prompt, Arc::clone(&openapi_actions));

    info!(
        plugin_count = openapi_specs.len(),
        action_count = openapi_actions.function_names().len(),
        "agent runtime started"
    );

    tokio::spawn(watch_inbound_realtime(
        config.clone(),
        inbound_notify.clone(),
    ));

    loop {
        let client = match pool.get().await {
            Ok(client) => client,
            Err(err) => {
                warn!(error = %err, "failed to get database connection");
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        let Some(inbound_message) = claim_next_channel_message()
            .bind(
                &client,
                &ChannelType::telegram,
                &ChannelMessageDirection::inbound,
                &ChannelMessageStatus::pending,
                &ChannelMessageStatus::processing,
            )
            .opt()
            .await?
        else {
            wait_for_work(&inbound_notify).await;
            continue;
        };

        let conversation_rows = list_conversation_messages()
            .bind(
                &client,
                &ChannelType::telegram,
                &inbound_message.external_conversation_id,
                &MAX_HISTORY_MESSAGES,
            )
            .all()
            .await?;

        let history = conversation_rows
            .into_iter()
            .filter(|message| message.id != inbound_message.id)
            .filter_map(|message| match message.direction {
                ChannelMessageDirection::inbound => Some(RigMessage::user(message.message_text)),
                ChannelMessageDirection::outbound => Some(RigMessage::assistant(message.message_text)),
            })
            .collect::<Vec<_>>();

        let (reply, inbound_status) = match agent.chat(&inbound_message.message_text, history).await {
            Ok(reply) => (reply, ChannelMessageStatus::processed),
            Err(err) => {
                warn!(
                    message_id = inbound_message.id,
                    error = %err,
                    "model request failed"
                );
                (format!("Model error: {err}"), ChannelMessageStatus::failed)
            }
        };

        insert_channel_message()
            .bind(
                &client,
                &ChannelType::telegram,
                &ChannelMessageDirection::outbound,
                &inbound_message.external_conversation_id,
                &Option::<String>::None,
                &Option::<String>::None,
                &reply,
                &ChannelMessageStatus::pending,
                &json!({
                    "source_message_id": inbound_message.id,
                }),
            )
            .one()
            .await?;

        update_channel_message_status()
            .bind(&client, &inbound_status, &inbound_message.id)
            .one()
            .await?;

        info!(
            inbound_message_id = inbound_message.id,
            conversation_id = inbound_message.external_conversation_id,
            "processed inbound message"
        );
    }
}

async fn watch_inbound_realtime(config: Config, inbound_notify: Arc<Notify>) {
    let realtime = match RealtimeClient::new(config.stack_api_url, config.service_role_jwt) {
        Ok(realtime) => realtime,
        Err(err) => {
            warn!(error = %err, "failed to build realtime client");
            return;
        }
    };

    if let Err(err) = realtime.connect().await {
        warn!(error = %err, "failed to connect to realtime");
        return;
    }

    let channel = realtime
        .channel("agent-inbound")
        .on_postgres_changes(
            PostgresChangesEvent::Insert,
            PostgresChangesFilter::new("public", "channel_messages")
                .with_filter("direction=eq.inbound"),
            move |_payload| {
                inbound_notify.notify_one();
            },
        )
        .subscribe(|status, err| {
            if let Some(err) = err {
                warn!(%status, error = %err, "realtime subscription status");
            } else {
                info!(%status, "realtime subscription status");
            }
        })
        .await;

    match channel {
        Ok(_channel) => std::future::pending::<()>().await,
        Err(err) => warn!(error = %err, "failed to subscribe to inbound realtime"),
    }
}

async fn wait_for_work(notify: &Notify) {
    tokio::select! {
        _ = notify.notified() => {}
        _ = tokio::time::sleep(Duration::from_secs(5)) => {}
    }
}
