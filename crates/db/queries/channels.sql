--: ChannelMessage()

--! insert_channel_message (external_user_id?, external_message_id?) : ChannelMessage
WITH selected_channel AS (
    SELECT
        c.org_id,
        c.created_by_user_id
    FROM public.channels c
    WHERE c.kind::TEXT = :channel::TEXT
    ORDER BY c.created_at ASC
    LIMIT 1
),
existing_conversation AS (
    SELECT
        c.id
    FROM public.conversations c
    WHERE c.title = :external_conversation_id::TEXT
    ORDER BY c.created_at ASC
    LIMIT 1
),
inserted_conversation AS (
    INSERT INTO public.conversations (
        org_id,
        created_by_user_id,
        title
    )
    SELECT
        sc.org_id,
        sc.created_by_user_id,
        :external_conversation_id::TEXT
    FROM selected_channel sc
    WHERE NOT EXISTS (SELECT 1 FROM existing_conversation)
    RETURNING id
),
resolved_conversation AS (
    SELECT id FROM existing_conversation
    UNION ALL
    SELECT id FROM inserted_conversation
),
inserted_message AS (
    INSERT INTO public.messages (
        conversation_id,
        role,
        content,
        metadata_json
    )
    SELECT
        rc.id,
        CASE
            WHEN :direction::TEXT = 'inbound' THEN 'user'::message_role
            ELSE 'assistant'::message_role
        END,
        :message_text::TEXT,
        COALESCE(:metadata_json::JSONB, '{}'::JSONB)
            || jsonb_build_object(
                'channel', :channel::TEXT,
                'direction', :direction::TEXT,
                'external_conversation_id', :external_conversation_id::TEXT,
                'external_user_id', :external_user_id::TEXT,
                'external_message_id', :external_message_id::TEXT,
                'status', :status::TEXT,
                'updated_at', NOW()
            )
    FROM resolved_conversation rc
    RETURNING
        id,
        content,
        metadata_json,
        created_at
)
SELECT
    m.id,
    m.metadata_json ->> 'channel' AS channel,
    m.metadata_json ->> 'direction' AS direction,
    m.metadata_json ->> 'external_conversation_id' AS external_conversation_id,
    m.content AS message_text,
    m.metadata_json ->> 'status' AS status,
    m.created_at,
    COALESCE((m.metadata_json ->> 'updated_at')::TIMESTAMPTZ, m.created_at) AS updated_at
FROM inserted_message m;

--! update_channel_message_status : ChannelMessage
UPDATE public.messages
SET
    metadata_json = CASE
        WHEN :status::TEXT = 'processed' THEN
            jsonb_set(
                jsonb_set(
                    jsonb_set(
                        metadata_json,
                        '{status}',
                        to_jsonb(:status::TEXT),
                        true
                    ),
                    '{processed_at}',
                    to_jsonb(NOW()),
                    true
                ),
                '{updated_at}',
                to_jsonb(NOW()),
                true
            )
        WHEN :status::TEXT = 'sent' THEN
            jsonb_set(
                jsonb_set(
                    jsonb_set(
                        metadata_json,
                        '{status}',
                        to_jsonb(:status::TEXT),
                        true
                    ),
                    '{delivered_at}',
                    to_jsonb(NOW()),
                    true
                ),
                '{updated_at}',
                to_jsonb(NOW()),
                true
            )
        ELSE
            jsonb_set(
                jsonb_set(
                    metadata_json,
                    '{status}',
                    to_jsonb(:status::TEXT),
                    true
                ),
                '{updated_at}',
                to_jsonb(NOW()),
                true
            )
    END
WHERE id = :id::UUID
RETURNING
    id,
    metadata_json ->> 'channel' AS channel,
    metadata_json ->> 'direction' AS direction,
    metadata_json ->> 'external_conversation_id' AS external_conversation_id,
    content AS message_text,
    metadata_json ->> 'status' AS status,
    created_at,
    COALESCE((metadata_json ->> 'updated_at')::TIMESTAMPTZ, created_at) AS updated_at;

--! claim_next_channel_message : ChannelMessage
WITH next_message AS (
    SELECT id
    FROM public.messages
    WHERE (metadata_json ->> 'channel') = :channel::TEXT
      AND (metadata_json ->> 'direction') = :direction::TEXT
      AND (metadata_json ->> 'status') = :from_status::TEXT
    ORDER BY created_at ASC
    LIMIT 1
    FOR UPDATE SKIP LOCKED
)
UPDATE public.messages
SET
    metadata_json = jsonb_set(
        jsonb_set(
            metadata_json,
            '{status}',
            to_jsonb(:to_status::TEXT),
            true
        ),
        '{updated_at}',
        to_jsonb(NOW()),
        true
    )
WHERE id IN (SELECT id FROM next_message)
RETURNING
    id,
    metadata_json ->> 'channel' AS channel,
    metadata_json ->> 'direction' AS direction,
    metadata_json ->> 'external_conversation_id' AS external_conversation_id,
    content AS message_text,
    metadata_json ->> 'status' AS status,
    created_at,
    COALESCE((metadata_json ->> 'updated_at')::TIMESTAMPTZ, created_at) AS updated_at;

--: ConversationMessage()

--! list_conversation_messages : ConversationMessage
SELECT
    recent_messages.id,
    recent_messages.direction,
    recent_messages.message_text,
    recent_messages.status,
    recent_messages.created_at
FROM (
    SELECT
        m.id,
        m.metadata_json ->> 'direction' AS direction,
        m.content AS message_text,
        m.metadata_json ->> 'status' AS status,
        m.created_at
    FROM public.messages m
    INNER JOIN public.conversations c
        ON c.id = m.conversation_id
    WHERE (m.metadata_json ->> 'channel') = :channel::TEXT
      AND c.title = :external_conversation_id::TEXT
    ORDER BY m.created_at DESC
    LIMIT :message_limit::BIGINT
) AS recent_messages
ORDER BY recent_messages.created_at ASC;
