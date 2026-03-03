-- migrate:up
-- =========================
-- CONVERSATIONS + MESSAGES (PRIVATE)
-- =========================
--
-- Core chat domain model (ChatGPT-style), private by default.
--
-- - Only the creating user can SELECT their conversations.
-- - Messages inherit privacy via the parent conversation.
--
-- Channels (Telegram etc.) are modeled separately and can be linked later.

CREATE TYPE message_role AS ENUM (
    'system',
    'user',
    'assistant',
    'tool'
);

COMMENT ON TYPE message_role IS
'Role of a message within a conversation. tool is for tool call results or tool output.';

CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    agent_id UUID REFERENCES public.agents(id) ON DELETE SET NULL,

    title TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE conversations IS
'Private chat threads. Only the creating user can access them (RLS enforced).';

CREATE INDEX conversations_user_created_at_idx
    ON conversations (created_by_user_id, created_at DESC);

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,

    role message_role NOT NULL,
    content TEXT NOT NULL,

    metadata_json JSONB NOT NULL DEFAULT '{}'::jsonb,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE messages IS
'Messages within a private conversation. Privacy is enforced via the parent conversation.';

CREATE INDEX messages_conversation_created_at_idx
    ON messages (conversation_id, created_at ASC, id ASC);

-- =========================
-- RLS
-- =========================

ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;

-- Conversations: only the creator can read/write their conversations.
CREATE POLICY conversations_select_own
ON conversations
FOR SELECT
USING (created_by_user_id = auth.uid());

CREATE POLICY conversations_insert_own
ON conversations
FOR INSERT
WITH CHECK (created_by_user_id = auth.uid());

CREATE POLICY conversations_update_own
ON conversations
FOR UPDATE
USING (created_by_user_id = auth.uid())
WITH CHECK (created_by_user_id = auth.uid());

CREATE POLICY conversations_delete_own
ON conversations
FOR DELETE
USING (created_by_user_id = auth.uid());

-- Messages: only visible if the parent conversation belongs to the current user.
CREATE POLICY messages_select_own_conversation
ON messages
FOR SELECT
USING (
    EXISTS (
        SELECT 1
        FROM conversations c
        WHERE c.id = conversation_id
          AND c.created_by_user_id = auth.uid()
    )
);

-- Messages: allow inserts only into your own conversations.
-- You can tighten role rules here if you want (e.g. only role='user').
CREATE POLICY messages_insert_own_conversation
ON messages
FOR INSERT
WITH CHECK (
    EXISTS (
        SELECT 1
        FROM conversations c
        WHERE c.id = conversation_id
          AND c.created_by_user_id = auth.uid()
    )
);

CREATE POLICY messages_update_own_conversation
ON messages
FOR UPDATE
USING (
    EXISTS (
        SELECT 1
        FROM conversations c
        WHERE c.id = conversation_id
          AND c.created_by_user_id = auth.uid()
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1
        FROM conversations c
        WHERE c.id = conversation_id
          AND c.created_by_user_id = auth.uid()
    )
);

CREATE POLICY messages_delete_own_conversation
ON messages
FOR DELETE
USING (
    EXISTS (
        SELECT 1
        FROM conversations c
        WHERE c.id = conversation_id
          AND c.created_by_user_id = auth.uid()
    )
);

-- migrate:down
DROP POLICY IF EXISTS messages_delete_own_conversation ON messages;
DROP POLICY IF EXISTS messages_update_own_conversation ON messages;
DROP POLICY IF EXISTS messages_insert_own_conversation ON messages;
DROP POLICY IF EXISTS messages_select_own_conversation ON messages;

DROP POLICY IF EXISTS conversations_delete_own ON conversations;
DROP POLICY IF EXISTS conversations_update_own ON conversations;
DROP POLICY IF EXISTS conversations_insert_own ON conversations;
DROP POLICY IF EXISTS conversations_select_own ON conversations;

DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS conversations;

DROP TYPE IF EXISTS message_role;