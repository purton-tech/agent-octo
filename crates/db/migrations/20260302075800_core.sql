-- migrate:up
CREATE TYPE resource_visibility AS ENUM (
    'private',
    'org'
);

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
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    visibility resource_visibility NOT NULL DEFAULT 'private',
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

CREATE TABLE agents (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    visibility resource_visibility NOT NULL DEFAULT 'private',
    name TEXT NOT NULL,
    system_prompt TEXT NOT NULL,
    default_connection_id UUID REFERENCES provider_connections(id),
    default_model TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE plugins (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    visibility resource_visibility NOT NULL DEFAULT 'private',
    name TEXT NOT NULL,
    openapi_spec JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE agent_plugins (
    agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    plugin_id UUID NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    PRIMARY KEY (agent_id, plugin_id)
);

CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    visibility resource_visibility NOT NULL DEFAULT 'private',
    kind TEXT NOT NULL,
    name TEXT NOT NULL,
    bot_token_secret_ref TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE agent_channels (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,
    channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    UNIQUE (agent_id, channel_id)
);

GRANT SELECT, INSERT, UPDATE ON channel_messages TO application_user;


-- migrate:down
DROP TABLE IF EXISTS agent_channels;
DROP TABLE IF EXISTS channels;
DROP TABLE IF EXISTS agent_plugins;
DROP TABLE IF EXISTS plugins;
DROP TABLE IF EXISTS agents;
DROP TABLE IF EXISTS channel_messages;
DROP TYPE IF EXISTS resource_visibility;
DROP TYPE IF EXISTS channel_message_status;
DROP TYPE IF EXISTS channel_message_direction;
DROP TYPE IF EXISTS channel_type;
