use std::sync::Arc;

use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai::Client;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let openai_client = Client::from_env();
    let agent = Arc::new(
        openai_client
            .agent("gpt-5-mini") // method provided by CompletionClient trait
            .preamble("You are a helpful assistant. Be very brief and concise")
            .name("Bob") // used in logging
            .build(),
    );
    let bot = Bot::new(std::env::var("TELEGRAM_BOT_TOKEN")?);

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = Arc::clone(&agent);
        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };

            let reply = match agent.prompt(text).await {
                Ok(reply) => reply,
                Err(err) => format!("Model error: {err}"),
            };

            bot.send_message(msg.chat.id, reply).await?;
            respond(())
        }
    })
    .await;

    Ok(())
}
