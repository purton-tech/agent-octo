use crate::config::Config;
use anyhow::{Context, anyhow};
use db::clorinde::deadpool_postgres::Pool;
use db::clorinde::queries::channels::{
    claim_next_channel_message, get_channel_config, update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use supabase_client_realtime::{PostgresChangesEvent, PostgresChangesFilter, RealtimeClient};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tokio::sync::Notify;
use tracing::{info, warn};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::new();
    let pool = db::create_pool(&config.database_url);
    let bot = Bot::new(load_telegram_bot_token(&pool).await?);
    let outbound_notify = std::sync::Arc::new(Notify::new());

    let outbound_task = tokio::spawn(drive_outbound_messages(
        bot.clone(),
        pool.clone(),
        outbound_notify.clone(),
    ));
    tokio::spawn(watch_outbound_realtime(config, outbound_notify));

    match outbound_task.await {
        Ok(Ok(())) => Err(anyhow!("outbound telegram worker exited unexpectedly")),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(err.into()),
    }
}

async fn load_telegram_bot_token(pool: &Pool) -> anyhow::Result<String> {
    let client = pool
        .get()
        .await
        .context("failed to get database connection for telegram bot startup")?;

    let channel_config = get_channel_config()
        .bind(&client, &ChannelType::telegram)
        .opt()
        .await
        .context("failed to load telegram channel configuration")?;

    let channel_config = channel_config.ok_or_else(|| anyhow!("no telegram channel configured"))?;

    Ok(channel_config.bot_token)
}

async fn drive_outbound_messages(
    bot: Bot,
    pool: Pool,
    outbound_notify: std::sync::Arc<Notify>,
) -> anyhow::Result<()> {
    loop {
        let client = pool.get().await?;

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
                return Err(err.into());
            }
        };

        let Some(message) = next_message else {
            outbound_notify.notified().await;
            continue;
        };

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

async fn watch_outbound_realtime(config: Config, outbound_notify: std::sync::Arc<Notify>) {
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
        .channel("telegram-outbound")
        .on_postgres_changes(
            PostgresChangesEvent::Insert,
            PostgresChangesFilter::new("public", "messages"),
            move |_payload| {
                outbound_notify.notify_one();
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
        Err(err) => warn!(error = %err, "failed to subscribe to outbound realtime"),
    }
}
