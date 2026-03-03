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
-- - bot_token_secret_ref points to a secret store entry (never store raw tokens).
--
-- Note:
--   - Routing a channel to an agent is handled in a separate join table migration.

CREATE TYPE channel_type AS ENUM (
    'telegram'
);

COMMENT ON TYPE channel_type IS
'Supported external channel integration types.';

CREATE TYPE channel_message_direction AS ENUM (
    'inbound',
    'outbound'
);

COMMENT ON TYPE channel_message_direction IS
'Direction of a channel message in the processing pipeline.';

CREATE TYPE channel_message_status AS ENUM (
    'pending',
    'processing',
    'processed',
    'sent',
    'failed'
);

COMMENT ON TYPE channel_message_status IS
'Lifecycle state for a channel-driven message in the processing pipeline.';

CREATE TABLE public.channels (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    visibility resource_visibility NOT NULL DEFAULT 'private',

    kind channel_type NOT NULL,               -- e.g. 'telegram'
    name TEXT NOT NULL,

    bot_token_secret_ref TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.channels IS
'External messaging integrations (e.g. Telegram bot). Secrets are stored externally and referenced here.';

COMMENT ON COLUMN public.channels.visibility IS
'private = only creator can see/use; org = visible to all org members (RLS enforced).';

COMMENT ON COLUMN public.channels.kind IS
'Channel integration type (e.g. telegram).';

COMMENT ON COLUMN public.channels.bot_token_secret_ref IS
'Reference to a secret store entry containing the bot token/credentials for this channel.';

CREATE INDEX channels_org_visibility_idx
    ON public.channels (org_id, visibility);

CREATE INDEX channels_creator_idx
    ON public.channels (created_by_user_id);

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

DROP TABLE IF EXISTS public.channels;
DROP TYPE IF EXISTS channel_message_status;
DROP TYPE IF EXISTS channel_message_direction;
DROP TYPE IF EXISTS channel_type;
