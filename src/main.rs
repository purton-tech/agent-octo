mod fetch_url;
mod monty_python;
mod openapi_actions;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use rig::client::{CompletionClient, ProviderClient};
use rig::completion::{Chat, Message as RigMessage};
use rig::providers::openai::Client;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tokio::sync::RwLock;
use tracing::{info, warn};

const SYSTEM_PROMPT: &str = include_str!("../SYSTEM_PROMPT.md");
const MAX_HISTORY_MESSAGES: usize = 20;

fn trim_history(history: &mut Vec<RigMessage>) {
    if history.len() > MAX_HISTORY_MESSAGES {
        let drop_count = history.len() - MAX_HISTORY_MESSAGES;
        history.drain(0..drop_count);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("workspace=info"))
        .with_target(false)
        .init();

    let openapi_specs =
        openapi_actions::OpenApiRegistry::load_specs_from_dir("/workspace/plugins")?;
    let openapi_actions = Arc::new(openapi_actions::OpenApiRegistry::from_specs(&openapi_specs));
    let system_prompt = {
        let dynamic_actions = openapi_actions.prompt_fragment();
        if dynamic_actions.is_empty() {
            SYSTEM_PROMPT.to_string()
        } else {
            format!("{SYSTEM_PROMPT}\n\n{dynamic_actions}")
        }
    };

    let openai_client = Client::from_env();
    let agent = Arc::new(
        openai_client
            .agent("gpt-5-mini") // method provided by CompletionClient trait
            .preamble(&system_prompt)
            .name("Bob") // used in logging
            .default_max_turns(4)
            .tool(monty_python::RunPython::new(Arc::clone(&openapi_actions)))
            .build(),
    );
    let histories = Arc::new(RwLock::new(HashMap::<i64, Vec<RigMessage>>::new()));
    let bot = Bot::new(std::env::var("TELEGRAM_BOT_TOKEN")?);
    info!(
        plugin_count = openapi_specs.len(),
        action_count = openapi_actions.function_names().len(),
        "telegram bot started"
    );

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = Arc::clone(&agent);
        let histories = Arc::clone(&histories);
        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };
            let chat_id = msg.chat.id;
            info!(chat_id = chat_id.0, "received telegram message");
            let typing_bot = bot.clone();
            let typing = tokio::spawn(async move {
                loop {
                    let _ = typing_bot
                        .send_chat_action(chat_id, ChatAction::Typing)
                        .await;
                    tokio::time::sleep(Duration::from_secs(4)).await;
                }
            });

            let history = {
                let histories = histories.read().await;
                histories.get(&chat_id.0).cloned().unwrap_or_default()
            };

            let reply = match agent.chat(text, history).await {
                Ok(reply) => {
                    let mut histories = histories.write().await;
                    let history = histories.entry(chat_id.0).or_default();
                    history.push(RigMessage::user(text));
                    history.push(RigMessage::assistant(reply.clone()));
                    trim_history(history);
                    reply
                }
                Err(err) => {
                    warn!(chat_id = chat_id.0, error = %err, "model request failed");
                    format!("Model error: {err}")
                }
            };
            typing.abort();

            bot.send_message(chat_id, reply).await?;
            info!(chat_id = chat_id.0, "sent telegram reply");
            respond(())
        }
    })
    .await;

    Ok(())
}
