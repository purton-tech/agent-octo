use crate::telegram::ingress::{InboundTelegramMessage, store_inbound_message};
use anyhow::{Context, anyhow};
use db::clorinde::deadpool_postgres::Pool;
use db::clorinde::queries::channels::{TelegramChannelConfig, list_telegram_channel_configs};
use db::clorinde::types::ChannelType;
use teloxide::prelude::*;
use tokio::task::JoinSet;
use tracing::{info, warn};

pub async fn run() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = db::create_pool(&database_url);
    let client = pool
        .get()
        .await
        .context("failed to get database connection for telegram startup")?;

    let channel_configs = list_telegram_channel_configs()
        .bind(&client, &ChannelType::telegram)
        .all()
        .await
        .context("failed to load telegram channel configuration")?;

    if channel_configs.is_empty() {
        return Err(anyhow!("no telegram channels configured"));
    }

    let mut pollers = JoinSet::new();

    for channel_config in channel_configs {
        pollers.spawn(run_channel_poller(pool.clone(), channel_config));
    }

    let result = pollers
        .join_next()
        .await
        .ok_or_else(|| anyhow!("no telegram channel pollers running"))?;

    match result {
        Ok(Ok(())) => Err(anyhow!("telegram channel poller exited unexpectedly")),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(err.into()),
    }
}

async fn run_channel_poller(
    pool: Pool,
    channel_config: TelegramChannelConfig,
) -> anyhow::Result<()> {
    let channel_id = channel_config.id;
    let bot = Bot::new(channel_config.bot_token);

    info!(channel_id = %channel_id, "telegram polling ingress started");

    teloxide::repl(bot, move |_bot: Bot, msg: Message| {
        let pool = pool.clone();

        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };

            let inbound = InboundTelegramMessage {
                channel_id,
                chat_id: msg.chat.id.0.to_string(),
                external_user_id: msg.from.as_ref().map(|user| user.id.0.to_string()),
                external_message_id: Some(msg.id.0.to_string()),
                text: text.to_owned(),
            };

            if let Err(err) = store_inbound_message(&pool, &inbound).await {
                warn!(
                    channel_id = %inbound.channel_id,
                    chat_id = inbound.chat_id,
                    error = %err,
                    "failed to process inbound telegram message"
                );
            }

            respond(())
        }
    })
    .await;

    Ok(())
}
