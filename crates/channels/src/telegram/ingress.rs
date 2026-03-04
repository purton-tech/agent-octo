use anyhow::Context;
use db::clorinde::deadpool_postgres::Pool;
use db::clorinde::queries::channels::{
    get_or_create_channel_conversation_for_channel, insert_channel_message,
};
use db::clorinde::types::{ChannelMessageDirection, ChannelMessageStatus};
use tracing::info;

#[derive(Clone, Debug)]
pub struct InboundTelegramMessage {
    pub channel_id: uuid::Uuid,
    pub chat_id: String,
    pub external_user_id: Option<String>,
    pub external_message_id: Option<String>,
    pub text: String,
}

pub async fn store_inbound_message(
    pool: &Pool,
    inbound: &InboundTelegramMessage,
) -> anyhow::Result<()> {
    let client = pool
        .get()
        .await
        .context("failed to get database connection")?;

    let channel_conversation = get_or_create_channel_conversation_for_channel()
        .bind(
            &client,
            &inbound.channel_id,
            &inbound.external_user_id,
            &inbound.chat_id,
        )
        .opt()
        .await
        .context("failed to resolve channel conversation")?
        .context("no channel routing configured for inbound telegram message")?;

    let message = insert_channel_message()
        .bind(
            &client,
            &inbound.external_message_id,
            &channel_conversation.id,
            &ChannelMessageDirection::inbound,
            &inbound.text,
            &ChannelMessageStatus::pending,
        )
        .one()
        .await
        .context("failed to store inbound telegram message")?;

    info!(
        message_id = %message.id,
        conversation_id = %message.conversation_id,
        channel_id = %inbound.channel_id,
        "stored inbound telegram message"
    );

    Ok(())
}
