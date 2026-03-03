--: ChannelConversation()

--! get_or_create_channel_conversation (external_user_id?) : ChannelConversation
WITH selected_channel AS (
    SELECT
        c.id,
        c.org_id,
        c.created_by_user_id,
        c.default_agent_id
    FROM public.channels c
    WHERE c.kind = :channel::channel_type
    ORDER BY c.created_at ASC
    LIMIT 1
),
updated_binding AS (
    UPDATE public.channel_conversations cc
    SET
        external_user_id = COALESCE(:external_user_id::TEXT, cc.external_user_id),
        updated_at = NOW()
    FROM selected_channel sc
    WHERE cc.channel_id = sc.id
      AND cc.external_conversation_id = :external_conversation_id::TEXT
    RETURNING
        cc.id,
        cc.channel_id,
        cc.conversation_id,
        cc.external_conversation_id
),
inserted_conversation AS (
    INSERT INTO public.conversations (
        org_id,
        created_by_user_id,
        agent_id,
        title
    )
    SELECT
        sc.org_id,
        sc.created_by_user_id,
        sc.default_agent_id,
        NULL
    FROM selected_channel sc
    WHERE sc.default_agent_id IS NOT NULL
      AND NOT EXISTS (SELECT 1 FROM updated_binding)
    RETURNING id
),
inserted_binding AS (
    INSERT INTO public.channel_conversations (
        channel_id,
        conversation_id,
        external_conversation_id,
        external_user_id
    )
    SELECT
        sc.id,
        ic.id,
        :external_conversation_id::TEXT,
        :external_user_id::TEXT
    FROM selected_channel sc
    INNER JOIN inserted_conversation ic
        ON TRUE
    RETURNING
        id,
        channel_id,
        conversation_id,
        external_conversation_id
),
resolved_binding AS (
    SELECT * FROM updated_binding
    UNION ALL
    SELECT * FROM inserted_binding
)
SELECT
    rb.id,
    rb.channel_id,
    rb.conversation_id,
    rb.external_conversation_id,
    c.agent_id
FROM resolved_binding rb
INNER JOIN public.conversations c
    ON c.id = rb.conversation_id
LIMIT 1;

--: ChannelMessage()

--! insert_channel_message (external_message_id?) : ChannelMessage
WITH updated_binding AS (
    UPDATE public.channel_conversations cc
    SET
        last_external_message_id = COALESCE(
            :external_message_id::TEXT,
            cc.last_external_message_id
        ),
        updated_at = NOW()
    WHERE cc.id = :channel_conversation_id::UUID
    RETURNING
        cc.id,
        cc.conversation_id,
        cc.external_conversation_id
),
inserted_message AS (
    INSERT INTO public.messages (
        conversation_id,
        role,
        content,
        channel_conversation_id,
        channel_message_direction,
        channel_message_status,
        external_message_id
    )
    SELECT
        ub.conversation_id,
        CASE
            WHEN :direction::channel_message_direction = 'inbound' THEN 'user'::message_role
            ELSE 'assistant'::message_role
        END,
        :message_text::TEXT,
        ub.id,
        :direction::channel_message_direction,
        :status::channel_message_status,
        :external_message_id::TEXT
    FROM updated_binding ub
    RETURNING
        id,
        conversation_id,
        channel_conversation_id,
        channel_message_direction,
        content,
        channel_message_status,
        created_at
)
SELECT
    im.id,
    im.conversation_id,
    im.channel_conversation_id,
    im.channel_message_direction AS direction,
    ub.external_conversation_id,
    im.content AS message_text,
    im.channel_message_status AS status,
    im.created_at
FROM inserted_message im
INNER JOIN updated_binding ub
    ON ub.id = im.channel_conversation_id;

--! update_channel_message_status : ChannelMessage
UPDATE public.messages m
SET
    channel_message_status = :status::channel_message_status
FROM public.channel_conversations cc
WHERE m.id = :id::UUID
  AND cc.id = m.channel_conversation_id
RETURNING
    m.id,
    m.conversation_id,
    m.channel_conversation_id,
    m.channel_message_direction AS direction,
    cc.external_conversation_id,
    m.content AS message_text,
    m.channel_message_status AS status,
    m.created_at;

--! claim_next_channel_message : ChannelMessage
WITH next_message AS (
    SELECT m.id
    FROM public.messages m
    INNER JOIN public.channel_conversations cc
        ON cc.id = m.channel_conversation_id
    INNER JOIN public.channels c
        ON c.id = cc.channel_id
    WHERE c.kind = :channel::channel_type
      AND m.channel_message_direction = :direction::channel_message_direction
      AND m.channel_message_status = :from_status::channel_message_status
    ORDER BY m.created_at ASC
    LIMIT 1
    FOR UPDATE OF m SKIP LOCKED
)
UPDATE public.messages m
SET
    channel_message_status = :to_status::channel_message_status
FROM public.channel_conversations cc
WHERE m.id IN (SELECT id FROM next_message)
  AND cc.id = m.channel_conversation_id
RETURNING
    m.id,
    m.conversation_id,
    m.channel_conversation_id,
    m.channel_message_direction AS direction,
    cc.external_conversation_id,
    m.content AS message_text,
    m.channel_message_status AS status,
    m.created_at;

--: ConversationMessage()

--! list_conversation_messages : ConversationMessage
SELECT
    recent_messages.id,
    recent_messages.direction,
    recent_messages.message_text,
    recent_messages.created_at
FROM (
    SELECT
        m.id,
        m.channel_message_direction AS direction,
        m.content AS message_text,
        m.created_at
    FROM public.messages m
    WHERE m.conversation_id = :conversation_id::UUID
      AND m.channel_message_direction IS NOT NULL
    ORDER BY m.created_at DESC
    LIMIT :message_limit::BIGINT
) AS recent_messages
ORDER BY recent_messages.created_at ASC;
