use std::time::Duration;

use crate::config::Config;
use db::clorinde::deadpool_postgres::Pool;
use db::clorinde::queries::channels::{
    claim_next_channel_message, insert_channel_message, update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use serde_json::json;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tracing::{info, warn};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::new();
    let bot = Bot::new(config.telegram_bot_token);
    let pool = db::create_pool(&config.application_url);

    tokio::spawn(drive_outbound_messages(bot.clone(), pool.clone()));

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

async fn drive_outbound_messages(bot: Bot, pool: Pool) {
    loop {
        let client = match pool.get().await {
            Ok(client) => client,
            Err(err) => {
                warn!(error = %err, "failed to get database connection");
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        let next_message = match claim_next_channel_message()
            .bind(
                &client,
                &ChannelType::telegram,
                &ChannelMessageDirection::outbound,
                &ChannelMessageStatus::pending,
                &ChannelMessageStatus::processing,
            )
            .opt()
            .await
        {
            Ok(message) => message,
            Err(err) => {
                warn!(error = %err, "failed to claim outbound telegram message");
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        let Some(message) = next_message else {
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue;
        };

        let chat_id = match message.external_conversation_id.parse::<i64>() {
            Ok(chat_id) => chat_id,
            Err(err) => {
                warn!(
                    message_id = message.id,
                    error = %err,
                    "invalid telegram chat id"
                );

                let _ = update_channel_message_status()
                    .bind(&client, &ChannelMessageStatus::failed, &message.id)
                    .one()
                    .await;

                continue;
            }
        };

        let send_result = bot
            .send_message(ChatId(chat_id), message.message_text.clone())
            .await;

        let new_status = if send_result.is_ok() {
            ChannelMessageStatus::sent
        } else {
            ChannelMessageStatus::failed
        };

        if let Err(err) = send_result {
            warn!(
                message_id = message.id,
                chat_id,
                error = %err,
                "failed to send telegram message"
            );
        } else {
            info!(message_id = message.id, chat_id, "sent telegram reply");
        }

        if let Err(err) = update_channel_message_status()
            .bind(&client, &new_status, &message.id)
            .one()
            .await
        {
            warn!(
                message_id = message.id,
                error = %err,
                "failed to update telegram message status"
            );
        }
    }
}
