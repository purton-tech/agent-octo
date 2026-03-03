use std::time::Duration;

use crate::config::Config;
use db::clorinde::deadpool_postgres::Pool;
use db::clorinde::queries::channels::{
    claim_next_channel_message, get_or_create_channel_conversation, insert_channel_message,
    update_channel_message_status,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus, ChannelType};
use supabase_client_realtime::{PostgresChangesEvent, PostgresChangesFilter, RealtimeClient};
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tokio::sync::Notify;
use tracing::{info, warn};

pub async fn run() -> anyhow::Result<()> {
    let config = Config::new();
    let bot = Bot::new(config.telegram_bot_token.clone());
    let pool = db::create_pool(&config.application_url);
    let outbound_notify = std::sync::Arc::new(Notify::new());

    tokio::spawn(drive_outbound_messages(
        bot.clone(),
        pool.clone(),
        outbound_notify.clone(),
    ));
    tokio::spawn(watch_outbound_realtime(config.clone(), outbound_notify));

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

            let client = match pool.get().await {
                Ok(client) => client,
                Err(err) => {
                    warn!(error = %err, "failed to get database connection");
                    return respond(());
                }
            };

            let channel_conversation = match get_or_create_channel_conversation()
                .bind(&client, &ChannelType::telegram, &external_user_id, &chat_id)
                .opt()
                .await
            {
                Ok(Some(channel_conversation)) => channel_conversation,
                Ok(None) => {
                    warn!(
                        chat_id = msg.chat.id.0,
                        "no channel routing configured for inbound telegram message"
                    );
                    return respond(());
                }
                Err(err) => {
                    warn!(
                        chat_id = msg.chat.id.0,
                        error = %err,
                        "failed to resolve channel conversation"
                    );
                    return respond(());
                }
            };

            match insert_channel_message()
                .bind(
                    &client,
                    &external_message_id,
                    &channel_conversation.id,
                    &ChannelMessageDirection::inbound,
                    &text,
                    &ChannelMessageStatus::pending,
                )
                .one()
                .await
            {
                Ok(message) => {
                    info!(
                        message_id = %message.id,
                        conversation_id = %message.conversation_id,
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

async fn drive_outbound_messages(bot: Bot, pool: Pool, outbound_notify: std::sync::Arc<Notify>) {
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
            wait_for_work(&outbound_notify).await;
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

async fn wait_for_work(notify: &Notify) {
    tokio::select! {
        _ = notify.notified() => {}
        _ = tokio::time::sleep(Duration::from_secs(5)) => {}
    }
}
