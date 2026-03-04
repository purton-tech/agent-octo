use std::collections::HashMap;
use std::time::Duration;

use db::clorinde::queries::channels::{
    claim_next_telegram_outbound_message, update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tokio::time::sleep;
use tracing::{info, warn};

const POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone)]
struct CachedBot {
    token: String,
    bot: Bot,
}

pub async fn run() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = db::create_pool(&database_url);

    drive_outbound_messages(pool).await
}

async fn drive_outbound_messages(
    pool: db::clorinde::deadpool_postgres::Pool,
) -> anyhow::Result<()> {
    let mut bots = HashMap::<uuid::Uuid, CachedBot>::new();

    loop {
        let client = pool.get().await?;

        let next_message = match claim_next_telegram_outbound_message()
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
                return Err(err.into());
            }
        };

        let Some(message) = next_message else {
            sleep(POLL_INTERVAL).await;
            continue;
        };

        let bot = bot_for_channel(&mut bots, message.channel_id, &message.bot_token);

        let chat_id = match message.external_conversation_id.parse::<i64>() {
            Ok(chat_id) => chat_id,
            Err(err) => {
                warn!(
                    message_id = %message.id,
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
                message_id = %message.id,
                chat_id,
                error = %err,
                "failed to send telegram message"
            );
        } else {
            info!(message_id = %message.id, chat_id, "sent telegram reply");
        }

        if let Err(err) = update_channel_message_status()
            .bind(&client, &new_status, &message.id)
            .one()
            .await
        {
            warn!(
                message_id = %message.id,
                error = %err,
                "failed to update telegram message status"
            );
        }
    }
}

fn bot_for_channel(
    bots: &mut HashMap<uuid::Uuid, CachedBot>,
    channel_id: uuid::Uuid,
    token: &str,
) -> Bot {
    let needs_refresh = bots
        .get(&channel_id)
        .map(|cached| cached.token != token)
        .unwrap_or(true);

    if needs_refresh {
        bots.insert(
            channel_id,
            CachedBot {
                token: token.to_owned(),
                bot: Bot::new(token.to_owned()),
            },
        );
    }

    bots.get(&channel_id)
        .map(|cached| cached.bot.clone())
        .expect("bot cache must contain channel")
}
