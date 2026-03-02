-- migrate:up
CREATE TYPE channel_type AS ENUM (
    'telegram'
);

CREATE TYPE channel_message_direction AS ENUM (
    'inbound',
    'outbound'
);

CREATE TYPE channel_message_status AS ENUM (
    'pending',
    'processing',
    'processed',
    'sent',
    'failed'
);

CREATE TABLE channel_messages (
    id BIGSERIAL PRIMARY KEY,
    channel channel_type NOT NULL,
    direction channel_message_direction NOT NULL,
    external_conversation_id TEXT NOT NULL,
    external_user_id TEXT,
    external_message_id TEXT,
    message_text TEXT NOT NULL,
    status channel_message_status NOT NULL DEFAULT 'pending',
    metadata_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    processed_at TIMESTAMP,
    delivered_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX channel_messages_inbound_idx
    ON channel_messages (channel, direction, status, created_at DESC);

CREATE INDEX channel_messages_conversation_idx
    ON channel_messages (channel, external_conversation_id, created_at DESC);


-- migrate:down
DROP TABLE IF EXISTS channel_messages;
DROP TYPE IF EXISTS channel_message_status;
DROP TYPE IF EXISTS channel_message_direction;
DROP TYPE IF EXISTS channel_type;
