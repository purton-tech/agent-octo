use std::sync::Arc;

use agent_runtime::config::Config;
use agent_runtime::provider;
use agent_runtime::system_prompt::SYSTEM_PROMPT;
use agent_runtime::{build_agent, build_system_prompt};
use db::clorinde::queries::channels::{
    claim_next_channel_message, insert_channel_message, list_conversation_messages,
    update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use rig::completion::{Chat, Message as RigMessage};
use supabase_client_realtime::{PostgresChangesEvent, PostgresChangesFilter, RealtimeClient};
use tokio::sync::Notify;
use tool_runtime::openapi_actions::OpenApiRegistry;
use tracing::{info, warn};

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
    let pool = db::create_pool(&config.database_url);
    let inbound_notify = Arc::new(Notify::new());

    let plugin_dir = format!("{}/../octo/plugins", env!("CARGO_MANIFEST_DIR"));
    let openapi_specs = OpenApiRegistry::load_specs_from_dir(&plugin_dir)?;
    let openapi_actions = Arc::new(OpenApiRegistry::from_specs(&openapi_specs));
    let system_prompt = build_system_prompt(SYSTEM_PROMPT, &openapi_actions);

    info!(
        plugin_count = openapi_specs.len(),
        action_count = openapi_actions.function_names().len(),
        "agent runtime started"
    );

    tokio::spawn(watch_inbound_realtime(
        config.clone(),
        inbound_notify.clone(),
    ));

    // Long-lived worker loop: wait for inbound work, claim one message, process it,
    // then repeat. This stays event-driven and lets the process handle messages
    // continuously until Kubernetes or the runtime stops it.
    loop {
        let client = pool.get().await?;

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
            // No pending inbound messages right now, so block until realtime tells
            // us a new message row was inserted.
            inbound_notify.notified().await;
            continue;
        };

        let conversation_rows = list_conversation_messages()
            .bind(
                &client,
                &inbound_message.conversation_id,
                &MAX_HISTORY_MESSAGES,
            )
            .all()
            .await?;

        let history = conversation_rows
            .into_iter()
            .filter(|message| message.id != inbound_message.id)
            .map(|message| match message.direction {
                ChannelMessageDirection::inbound => RigMessage::user(message.message_text),
                ChannelMessageDirection::outbound => RigMessage::assistant(message.message_text),
            })
            .collect::<Vec<_>>();

        let (reply, inbound_status) = match process_inbound_message(
            &client,
            &inbound_message.conversation_id,
            &inbound_message.message_text,
            history,
            &system_prompt,
            &openapi_actions,
        )
        .await
        {
            Ok(reply) => (reply, ChannelMessageStatus::processed),
            Err(err) => {
                warn!(
                    message_id = %inbound_message.id,
                    error = %err,
                    "model request failed"
                );
                (format!("Model error: {err}"), ChannelMessageStatus::failed)
            }
        };

        insert_channel_message()
            .bind(
                &client,
                &Option::<String>::None,
                &inbound_message.channel_conversation_id,
                &ChannelMessageDirection::outbound,
                &reply,
                &ChannelMessageStatus::pending,
            )
            .one()
            .await?;

        update_channel_message_status()
            .bind(&client, &inbound_status, &inbound_message.id)
            .one()
            .await?;

        info!(
            inbound_message_id = %inbound_message.id,
            conversation_id = %inbound_message.conversation_id,
            "processed inbound message"
        );
    }
}

async fn process_inbound_message(
    client: &db::clorinde::deadpool_postgres::Client,
    conversation_id: &uuid::Uuid,
    message_text: &str,
    history: Vec<RigMessage>,
    system_prompt: &str,
    openapi_actions: &Arc<OpenApiRegistry>,
) -> anyhow::Result<String> {
    let provider_config = provider::load_for_conversation(client, conversation_id).await?;
    let provider_client = provider::build_client(&provider_config)?;
    let agent = build_agent(
        provider_client,
        &provider_config.model,
        system_prompt.to_owned(),
        Arc::clone(openapi_actions),
    );

    agent.chat(message_text, history).await.map_err(Into::into)
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
            PostgresChangesFilter::new("public", "messages"),
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
