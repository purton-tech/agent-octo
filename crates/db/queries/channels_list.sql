--: ChannelCard()
--: ChannelSetup()

--! has_telegram_channel : ChannelSetup
SELECT EXISTS (
    SELECT 1
    FROM public.channels c
    WHERE c.org_id = public.b64url_to_uuid(:org_id::TEXT)
      AND c.kind = 'telegram'::channel_type
) AS configured;

--! connect_telegram_channel : ChannelSetup
WITH inserted AS (
    INSERT INTO public.channels (
        org_id,
        created_by_user_id,
        visibility,
        kind,
        name,
        bot_token,
        default_agent_id
    )
    SELECT
        public.b64url_to_uuid(:org_id::TEXT),
        auth.uid(),
        'private'::resource_visibility,
        'telegram'::channel_type,
        'Telegram',
        :bot_token::TEXT,
        a.id
    FROM public.agents a
    WHERE a.id = :default_agent_id::UUID
      AND a.org_id = public.b64url_to_uuid(:org_id::TEXT)
      AND (
          a.visibility = 'org'
          OR a.created_by_user_id = auth.uid()
      )
      AND NOT EXISTS (
          SELECT 1
          FROM public.channels c
          WHERE c.org_id = public.b64url_to_uuid(:org_id::TEXT)
            AND c.kind = 'telegram'::channel_type
      )
    RETURNING 1
)
SELECT EXISTS(SELECT 1 FROM inserted) AS configured;

--! list_org_channels : ChannelCard
SELECT
    id,
    name,
    kind::TEXT AS kind,
    visibility::TEXT AS visibility,
    updated_at
FROM public.channels
WHERE org_id = public.b64url_to_uuid(:org_id::TEXT)
  AND (
      visibility = 'org'
      OR created_by_user_id = auth.uid()
  )
ORDER BY updated_at DESC;
