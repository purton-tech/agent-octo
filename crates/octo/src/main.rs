use std::sync::Arc;
use std::time::Duration;

use agent_runtime::{ConversationStore, build_agent, build_system_prompt};
use rig::client::ProviderClient;
use rig::completion::Chat;
use rig::providers::openai::Client;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tool_runtime::openapi_actions::OpenApiRegistry;
use tracing::{info, warn};

const SYSTEM_PROMPT: &str = include_str!("../SYSTEM_PROMPT.md");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("octo=info"))
        .with_target(false)
        .init();

    let plugin_dir = format!("{}/plugins", env!("CARGO_MANIFEST_DIR"));
    let openapi_specs = OpenApiRegistry::load_specs_from_dir(&plugin_dir)?;
    let openapi_actions = Arc::new(OpenApiRegistry::from_specs(&openapi_specs));
    let system_prompt = build_system_prompt(SYSTEM_PROMPT, &openapi_actions);
    println!("System prompt:\n{system_prompt}\n");

    let openai_client = Client::from_env();
    let agent = Arc::new(build_agent(
        openai_client,
        system_prompt,
        Arc::clone(&openapi_actions),
    ));
    let conversations = Arc::new(ConversationStore::new());
    let bot = Bot::new(std::env::var("TELEGRAM_BOT_TOKEN")?);
    info!(
        plugin_count = openapi_specs.len(),
        action_count = openapi_actions.function_names().len(),
        "telegram bot started"
    );

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = Arc::clone(&agent);
        let conversations = Arc::clone(&conversations);
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

            let history = conversations.history(chat_id.0).await;
            let reply = match agent.chat(text, history).await {
                Ok(reply) => {
                    conversations.push_turn(chat_id.0, text, &reply).await;
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
