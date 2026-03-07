use crate::telegram::ingress::{InboundTelegramMessage, store_inbound_message};
use anyhow::Context;
use db::clorinde::deadpool_postgres::Pool;
use db::clorinde::queries::channels::{TelegramChannelConfig, list_telegram_channel_configs};
use db::clorinde::types::ChannelType;
use std::collections::HashMap;
use teloxide::prelude::*;
use tokio::{
    task::JoinHandle,
    time::{Duration, sleep},
};
use tracing::{info, warn};
use uuid::Uuid;

const CONFIG_REFRESH_INTERVAL: Duration = Duration::from_secs(10);
const POLLER_RESTART_BACKOFF: Duration = Duration::from_secs(5);

struct ActivePoller {
    bot_token: String,
    handle: JoinHandle<()>,
}

pub async fn run() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = db::create_pool(&database_url);
    let mut pollers: HashMap<Uuid, ActivePoller> = HashMap::new();
    let mut logged_empty = false;

    loop {
        let channel_configs = match load_telegram_configs(&pool).await {
            Ok(configs) => configs,
            Err(err) => {
                warn!(error = %err, "failed to refresh telegram channel configuration");
                sleep(CONFIG_REFRESH_INTERVAL).await;
                continue;
            }
        };

        let mut desired: HashMap<Uuid, TelegramChannelConfig> = HashMap::new();
        for config in channel_configs {
            desired.insert(config.id, config);
        }

        let active_ids: Vec<Uuid> = pollers.keys().copied().collect();
        for channel_id in active_ids {
            let should_stop = match (pollers.get(&channel_id), desired.get(&channel_id)) {
                (Some(active), Some(config)) => active.bot_token != config.bot_token,
                (_, None) => true,
                _ => false,
            };

            if should_stop && let Some(active) = pollers.remove(&channel_id) {
                active.handle.abort();
                info!(channel_id = %channel_id, "stopped telegram poller");
            }
        }

        for config in desired.values() {
            if let std::collections::hash_map::Entry::Vacant(entry) = pollers.entry(config.id) {
                let handle = tokio::spawn(run_channel_poller(pool.clone(), config.clone()));
                entry.insert(ActivePoller {
                    bot_token: config.bot_token.clone(),
                    handle,
                });
                info!(channel_id = %config.id, "started telegram poller");
            }
        }

        if desired.is_empty() {
            if !logged_empty {
                info!("no telegram channels configured; waiting for channel setup");
                logged_empty = true;
            }
        } else {
            logged_empty = false;
            info!(
                configured_channels = desired.len(),
                active_pollers = pollers.len(),
                "telegram ingress supervisor tick"
            );
        }

        sleep(CONFIG_REFRESH_INTERVAL).await;
    }
}

async fn load_telegram_configs(pool: &Pool) -> anyhow::Result<Vec<TelegramChannelConfig>> {
    let client = pool
        .get()
        .await
        .context("failed to get database connection for telegram config refresh")?;

    let configs = list_telegram_channel_configs()
        .bind(&client, &ChannelType::telegram)
        .all()
        .await
        .context("failed to load telegram channel configuration")?;

    Ok(configs)
}

async fn run_channel_poller(pool: Pool, channel_config: TelegramChannelConfig) {
    let channel_id = channel_config.id;
    let bot_token = channel_config.bot_token;

    loop {
        let bot = Bot::new(bot_token.clone());
        let poller_pool = pool.clone();
        info!(channel_id = %channel_id, "telegram polling ingress started");

        teloxide::repl(bot, move |_bot: Bot, msg: Message| {
            let pool = poller_pool.clone();

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

        warn!(
            channel_id = %channel_id,
            "telegram poller exited unexpectedly; restarting after backoff"
        );
        sleep(POLLER_RESTART_BACKOFF).await;
    }
}
