-- migrate:up
-- =========================
-- CHANNELS
-- =========================
--
-- Channels represent external messaging integrations (e.g. Telegram bots).
--
-- - Scoped to an org (tenant isolation).
-- - Created by a user.
-- - Private by default; can be shared to org.
-- - Can point at a default agent for new inbound external threads.
-- - bot_token stores the provider token for this channel.
--
-- Channel routing:
--   - channel_conversations binds an external thread to one internal conversation
--   - messages can participate in channel queue/delivery state

CREATE TYPE channel_type AS ENUM (
    'telegram'
);

COMMENT ON TYPE channel_type IS
'Supported external channel integration types.';

GRANT USAGE ON TYPE channel_type TO application_user;
GRANT USAGE ON TYPE channel_type TO application_readonly;

CREATE TYPE public.channel_message_direction AS ENUM (
    'inbound',
    'outbound'
);

COMMENT ON TYPE public.channel_message_direction IS
'Direction of a channel message in the processing pipeline.';

GRANT USAGE ON TYPE public.channel_message_direction TO application_user;
GRANT USAGE ON TYPE public.channel_message_direction TO application_readonly;

CREATE TYPE public.channel_message_status AS ENUM (
    'pending',
    'processing',
    'processed',
    'sent',
    'failed'
);

COMMENT ON TYPE public.channel_message_status IS
'Lifecycle state for a channel-driven message in the processing pipeline.';

GRANT USAGE ON TYPE public.channel_message_status TO application_user;
GRANT USAGE ON TYPE public.channel_message_status TO application_readonly;

CREATE TABLE public.channels (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    visibility resource_visibility NOT NULL DEFAULT 'private',

    kind channel_type NOT NULL,               -- e.g. 'telegram'
    name TEXT NOT NULL,
    default_agent_id UUID REFERENCES public.agents(id) ON DELETE SET NULL,

    bot_token TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.channels IS
'External messaging integrations (e.g. Telegram bot).';

COMMENT ON COLUMN public.channels.visibility IS
'private = only creator can see/use; org = visible to all org members (RLS enforced).';

COMMENT ON COLUMN public.channels.kind IS
'Channel integration type (e.g. telegram).';

COMMENT ON COLUMN public.channels.default_agent_id IS
'Default agent assigned to newly created conversations routed through this channel.';

COMMENT ON COLUMN public.channels.bot_token IS
'Bot token/credentials for this channel.';

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

CREATE INDEX channels_org_visibility_idx
    ON public.channels (org_id, visibility);

CREATE INDEX channels_creator_idx
    ON public.channels (created_by_user_id);

CREATE INDEX channel_conversations_channel_external_idx
    ON public.channel_conversations (channel_id, external_conversation_id);

CREATE INDEX messages_channel_queue_idx
    ON public.messages (channel_message_direction, channel_message_status, created_at)
    WHERE channel_conversation_id IS NOT NULL;

CREATE INDEX messages_channel_conversation_idx
    ON public.messages (channel_conversation_id, created_at ASC);

GRANT SELECT, INSERT, UPDATE, DELETE ON public.channels TO application_user;
GRANT SELECT ON public.channels TO application_readonly;

GRANT SELECT, INSERT, UPDATE, DELETE ON public.channel_conversations TO application_user;
GRANT SELECT ON public.channel_conversations TO application_readonly;

-- =========================
-- RLS
-- =========================

ALTER TABLE public.channels ENABLE ROW LEVEL SECURITY;

-- Read: org members can see org-visible channels, plus their own private channels.
CREATE POLICY channels_select
ON public.channels
FOR SELECT
USING (
    org.is_org_member(org_id)
    AND (
        visibility = 'org'
        OR created_by_user_id = auth.uid()
    )
);

-- Insert: must be in your org, and you must be the creator.
CREATE POLICY channels_insert
ON public.channels
FOR INSERT
WITH CHECK (
    org.is_org_member(org_id)
    AND created_by_user_id = auth.uid()
);

-- Update:
--   - Creator can update their private channel.
--   - Org admins can update org-visible channels.
CREATE POLICY channels_update
ON public.channels
FOR UPDATE
USING (
    org.is_org_member(org_id)
    AND (
        (visibility = 'private' AND created_by_user_id = auth.uid())
        OR (visibility = 'org' AND org.is_org_admin(org_id))
    )
)
WITH CHECK (
    org.is_org_member(org_id)
    AND (
        (visibility = 'private' AND created_by_user_id = auth.uid())
        OR (visibility = 'org' AND org.is_org_admin(org_id))
    )
);

-- Delete:
--   - Creator can delete their private channel.
--   - Org admins can delete org-visible channels.
CREATE POLICY channels_delete
ON public.channels
FOR DELETE
USING (
    org.is_org_member(org_id)
    AND (
        (visibility = 'private' AND created_by_user_id = auth.uid())
        OR (visibility = 'org' AND org.is_org_admin(org_id))
    )
);

-- migrate:down
DROP POLICY IF EXISTS channels_delete ON public.channels;
DROP POLICY IF EXISTS channels_update ON public.channels;
DROP POLICY IF EXISTS channels_insert ON public.channels;
DROP POLICY IF EXISTS channels_select ON public.channels;

DROP INDEX IF EXISTS public.messages_channel_conversation_idx;
DROP INDEX IF EXISTS public.messages_channel_queue_idx;
DROP INDEX IF EXISTS public.channel_conversations_channel_external_idx;

ALTER TABLE public.messages
DROP COLUMN IF EXISTS external_message_id,
DROP COLUMN IF EXISTS channel_message_status,
DROP COLUMN IF EXISTS channel_message_direction,
DROP COLUMN IF EXISTS channel_conversation_id;

DROP TABLE IF EXISTS public.channel_conversations;

DROP TABLE IF EXISTS public.channels;
DROP TYPE IF EXISTS public.channel_message_status;
DROP TYPE IF EXISTS public.channel_message_direction;
DROP TYPE IF EXISTS channel_type;
