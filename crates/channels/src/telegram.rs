use db::clorinde::queries::channels::insert_channel_message;
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use serde_json::json;
use teloxide::prelude::*;
use tracing::{info, warn};

pub async fn run() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")?;
    let bot = Bot::new(std::env::var("TELEGRAM_BOT_TOKEN")?);
    let pool = db::create_pool(&database_url);

    info!("telegram ingress started");

    teloxide::repl(bot, move |_bot: Bot, msg: Message| {
        let pool = pool.clone();

        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };

            let chat_id = msg.chat.id.0.to_string();
            let external_user_id = msg.from.as_ref().map(|user| user.id.0.to_string());
            let external_message_id = Some(msg.id.0.to_string());
            let metadata = json!({
                "telegram_chat_id": msg.chat.id.0,
                "telegram_message_id": msg.id.0,
            });

            let client = match pool.get().await {
                Ok(client) => client,
                Err(err) => {
                    warn!(error = %err, "failed to get database connection");
                    return respond(());
                }
            };

            match insert_channel_message()
                .bind(
                    &client,
                    &ChannelType::telegram,
                    &ChannelMessageDirection::inbound,
                    &chat_id,
                    &external_user_id,
                    &external_message_id,
                    &text,
                    &ChannelMessageStatus::pending,
                    &metadata,
                )
                .one()
                .await
            {
                Ok(message) => {
                    info!(
                        message_id = message.id,
                        conversation_id = message.external_conversation_id,
                        "stored inbound telegram message"
                    );
                }
                Err(err) => {
                    warn!(
                        chat_id = msg.chat.id.0,
                        error = %err,
                        "failed to store inbound telegram message"
                    );
                }
            }

            respond(())
        }
    })
    .await;

    Ok(())
}
