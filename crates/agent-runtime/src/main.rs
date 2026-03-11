use std::sync::Arc;
use std::time::Duration;

use agent_runtime::config::Config;
use agent_runtime::provider;
use agent_runtime::system_prompt::SYSTEM_PROMPT;
use agent_runtime::{build_agent, build_system_prompt};
use db::clorinde::queries::billing::record_llm_usage_for_conversation;
use db::clorinde::queries::channels::{
    claim_next_channel_message, insert_channel_message, list_conversation_messages,
    update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use rig::agent::{PromptRequest, PromptResponse};
use rig::completion::Message as RigMessage;
use tokio::time::sleep;
use tool_runtime::openapi_actions::OpenApiRegistry;
use tracing::{info, warn};

const MAX_HISTORY_MESSAGES: i64 = 20;
const POLL_INTERVAL: Duration = Duration::from_secs(1);

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

    let plugin_dir = format!("{}/../octo/plugins", env!("CARGO_MANIFEST_DIR"));
    let openapi_specs = OpenApiRegistry::load_specs_from_dir(&plugin_dir)?;
    let openapi_actions = Arc::new(OpenApiRegistry::from_specs(&openapi_specs));
    let system_prompt = build_system_prompt(SYSTEM_PROMPT, &openapi_actions);

    info!(
        plugin_count = openapi_specs.len(),
        action_count = openapi_actions.function_names().len(),
        "agent runtime started"
    );

    // Long-lived worker loop: wait for inbound work, claim one message, process it,
    // then repeat. If there is no work, sleep briefly before retrying.
    loop {
        let mut client = pool.get().await?;

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
            sleep(POLL_INTERVAL).await;
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

        let (reply, usage, inbound_status) = match process_inbound_message(
            &client,
            &inbound_message.conversation_id,
            &inbound_message.message_text,
            history,
            &system_prompt,
            &openapi_actions,
        )
        .await
        {
            Ok(response) => (
                response.output,
                Some(response.usage),
                ChannelMessageStatus::processed,
            ),
            Err(err) => {
                warn!(
                    message_id = %inbound_message.id,
                    error = %err,
                    "model request failed"
                );
                (
                    format!("Model error: {err}"),
                    None,
                    ChannelMessageStatus::failed,
                )
            }
        };

        let transaction = client.transaction().await?;

        if let Some(usage) = usage {
            record_llm_usage_for_conversation()
                .bind(
                    &transaction,
                    &inbound_message.conversation_id,
                    &(usage.input_tokens as i64),
                    &(usage.output_tokens as i64),
                )
                .one()
                .await?;
        }

        insert_channel_message()
            .bind(
                &transaction,
                &Option::<String>::None,
                &inbound_message.channel_conversation_id,
                &ChannelMessageDirection::outbound,
                &reply,
                &ChannelMessageStatus::pending,
            )
            .one()
            .await?;

        update_channel_message_status()
            .bind(&transaction, &inbound_status, &inbound_message.id)
            .one()
            .await?;

        transaction.commit().await?;

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
    mut history: Vec<RigMessage>,
    system_prompt: &str,
    openapi_actions: &Arc<OpenApiRegistry>,
) -> anyhow::Result<PromptResponse> {
    let provider_config = provider::load_for_conversation(client, conversation_id).await?;
    let provider_client = provider::build_client(&provider_config)?;
    let agent = build_agent(
        provider_client,
        &provider_config.model,
        system_prompt.to_owned(),
        Arc::clone(openapi_actions),
    );

    PromptRequest::from_agent(&agent, message_text)
        .with_history(&mut history)
        .extended_details()
        .await
        .map_err(Into::into)
}
