mod monty_python;

use std::sync::Arc;

use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai::Client;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("workspace=info"))
        .with_target(false)
        .init();

    let openai_client = Client::from_env();
    let agent = Arc::new(
        openai_client
            .agent("gpt-5-mini") // method provided by CompletionClient trait
            .preamble("You are a helpful assistant. Be very brief and concise. You can use the run_python tool for calculations or short code execution when helpful. Inside that tool, Python may call fetch(url) for HTTP(S) GET requests. fetch(url) returns the response body as text, not a response object, so use json.loads(fetch(url)) when the endpoint returns JSON.")
            .name("Bob") // used in logging
            .tool(monty_python::RunPython)
            .build(),
    );
    let bot = Bot::new(std::env::var("TELEGRAM_BOT_TOKEN")?);
    info!("telegram bot started");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = Arc::clone(&agent);
        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };
            info!(chat_id = msg.chat.id.0, "received telegram message");
            let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;

            let reply = match agent.prompt(text).await {
                Ok(reply) => reply,
                Err(err) => format!("Model error: {err}"),
            };

            if reply.starts_with("Model error: ") {
                warn!(chat_id = msg.chat.id.0, error = %reply, "model request failed");
            }
            bot.send_message(msg.chat.id, reply).await?;
            info!(chat_id = msg.chat.id.0, "sent telegram reply");
            respond(())
        }
    })
    .await;

    Ok(())
}
