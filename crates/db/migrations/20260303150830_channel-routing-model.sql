-- migrate:up
-- =========================
-- CHANNEL ROUTING MODEL
-- =========================
--
-- Adds the explicit binding between an external channel thread and an internal
-- conversation. This replaces the old implicit "channel_messages" queue model.
--
-- - channels can point at a default agent for new inbound threads
-- - channel_conversations maps an external thread to one conversation
-- - messages can optionally participate in channel delivery state

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_type
        WHERE typname = 'channel_message_direction'
          AND typnamespace = 'public'::regnamespace
    ) THEN
        CREATE TYPE public.channel_message_direction AS ENUM (
            'inbound',
            'outbound'
        );
    END IF;
END
$$;

COMMENT ON TYPE public.channel_message_direction IS
'Direction of a channel message in the processing pipeline.';

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_type
        WHERE typname = 'channel_message_status'
          AND typnamespace = 'public'::regnamespace
    ) THEN
        CREATE TYPE public.channel_message_status AS ENUM (
            'pending',
            'processing',
            'processed',
            'sent',
            'failed'
        );
    END IF;
END
$$;

COMMENT ON TYPE public.channel_message_status IS
'Lifecycle state for a channel-driven message in the processing pipeline.';

ALTER TABLE public.channels
ADD COLUMN default_agent_id UUID REFERENCES public.agents(id) ON DELETE SET NULL;

COMMENT ON COLUMN public.channels.default_agent_id IS
'Default agent assigned to newly created conversations routed through this channel.';

CREATE TABLE public.channel_conversations (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    channel_id UUID NOT NULL REFERENCES public.channels(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL UNIQUE REFERENCES public.conversations(id) ON DELETE CASCADE,
    external_conversation_id TEXT NOT NULL,
    external_user_id TEXT,
    last_external_message_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (channel_id, external_conversation_id)
);

COMMENT ON TABLE public.channel_conversations IS
'Binding between an external channel thread and an internal conversation.';

COMMENT ON COLUMN public.channel_conversations.external_conversation_id IS
'Provider-specific thread identifier (for example, a Telegram chat id).';

COMMENT ON COLUMN public.channel_conversations.external_user_id IS
'Provider-specific user identifier for the participant on the external channel.';

ALTER TABLE public.messages
ADD COLUMN channel_conversation_id UUID REFERENCES public.channel_conversations(id) ON DELETE SET NULL,
ADD COLUMN channel_message_direction public.channel_message_direction,
ADD COLUMN channel_message_status public.channel_message_status,
ADD COLUMN external_message_id TEXT;

COMMENT ON COLUMN public.messages.channel_conversation_id IS
'Optional link to the external channel thread this message came from or is destined for.';

COMMENT ON COLUMN public.messages.channel_message_direction IS
'Direction in the external channel flow. Null for non-channel messages.';

COMMENT ON COLUMN public.messages.channel_message_status IS
'Queue or delivery status for channel-driven messages. Null for non-channel messages.';

COMMENT ON COLUMN public.messages.external_message_id IS
'Provider-specific message identifier, when available.';

CREATE INDEX channel_conversations_channel_external_idx
    ON public.channel_conversations (channel_id, external_conversation_id);

CREATE INDEX messages_channel_queue_idx
    ON public.messages (channel_message_direction, channel_message_status, created_at)
    WHERE channel_conversation_id IS NOT NULL;

CREATE INDEX messages_channel_conversation_idx
    ON public.messages (channel_conversation_id, created_at ASC);


-- migrate:down
DROP INDEX IF EXISTS public.messages_channel_conversation_idx;
DROP INDEX IF EXISTS public.messages_channel_queue_idx;
DROP INDEX IF EXISTS public.channel_conversations_channel_external_idx;

ALTER TABLE public.messages
DROP COLUMN IF EXISTS external_message_id,
DROP COLUMN IF EXISTS channel_message_status,
DROP COLUMN IF EXISTS channel_message_direction,
DROP COLUMN IF EXISTS channel_conversation_id;

DROP TABLE IF EXISTS public.channel_conversations;

ALTER TABLE public.channels
DROP COLUMN IF EXISTS default_agent_id;

DROP TYPE IF EXISTS public.channel_message_status;
DROP TYPE IF EXISTS public.channel_message_direction;
