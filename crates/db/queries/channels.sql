--: ChannelMessage()

--! insert_channel_message (external_user_id?, external_message_id?) : ChannelMessage
INSERT INTO channel_messages (
    channel,
    direction,
    external_conversation_id,
    external_user_id,
    external_message_id,
    message_text,
    status,
    metadata_json
)
VALUES (
    :channel::channel_type,
    :direction::channel_message_direction,
    :external_conversation_id::TEXT,
    :external_user_id::TEXT,
    :external_message_id::TEXT,
    :message_text::TEXT,
    :status::channel_message_status,
    :metadata_json::JSONB
)
RETURNING
    id,
    channel,
    direction,
    external_conversation_id,
    message_text,
    status,
    created_at,
    updated_at;

--! update_channel_message_status : ChannelMessage
UPDATE channel_messages
SET
    status = :status::channel_message_status,
    processed_at = CASE
        WHEN :status::channel_message_status = 'processed' THEN NOW()
        ELSE processed_at
    END,
    delivered_at = CASE
        WHEN :status::channel_message_status = 'sent' THEN NOW()
        ELSE delivered_at
    END,
    updated_at = NOW()
WHERE id = :id::BIGINT
RETURNING
    id,
    channel,
    direction,
    external_conversation_id,
    message_text,
    status,
    created_at,
    updated_at;

--: ConversationMessage()

--! list_conversation_messages : ConversationMessage
SELECT
    id,
    direction,
    message_text,
    status,
    created_at
FROM channel_messages
WHERE channel = :channel::channel_type
  AND external_conversation_id = :external_conversation_id::TEXT
ORDER BY created_at ASC
LIMIT :message_limit::BIGINT;
