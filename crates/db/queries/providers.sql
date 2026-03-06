--: ResolvedProviderConfig()
--: ProviderConnectionCard()
--: ProviderConnectionSetup()

--! list_provider_connections : ProviderConnectionCard
SELECT
    p.id,
    p.name AS provider_kind,
    p.default_model_display_name AS display_name,
    p.base_url,
    p.default_model_name AS default_model,
    MAX(al.created_at) AS updated_at
FROM public.agent_llm al
INNER JOIN public.agents a
    ON a.id = al.agent_id
INNER JOIN public.providers p
    ON p.id = al.provider_id
WHERE a.org_id = public.b64url_to_uuid(:org_id::TEXT)
GROUP BY
    p.id,
    p.name,
    p.default_model_display_name,
    p.base_url,
    p.default_model_name
ORDER BY updated_at DESC;

--! create_provider_connection : ProviderConnectionSetup
WITH selected_provider AS (
    SELECT p.id
    FROM public.providers p
    WHERE p.name = :provider_kind::TEXT
    LIMIT 1
),
target_agents AS (
    SELECT a.id
    FROM public.agents a
    WHERE a.org_id = public.b64url_to_uuid(:org_id::TEXT)
      AND NOT EXISTS (
          SELECT 1
          FROM public.agent_llm al
          WHERE al.agent_id = a.id
      )
),
inserted AS (
    INSERT INTO public.agent_llm (
        agent_id,
        provider_id,
        api_key,
        model_name
    )
    SELECT
        ta.id,
        sp.id,
        :api_key::TEXT,
        NULL
    FROM target_agents ta
    CROSS JOIN selected_provider sp
    RETURNING 1
)
SELECT EXISTS(SELECT 1 FROM inserted) AS configured;

--! get_provider_for_conversation : ResolvedProviderConfig
SELECT
    p.id AS connection_id,
    p.name AS provider_kind,
    al.api_key,
    p.base_url,
    COALESCE(al.model_name, p.default_model_name) AS model
FROM public.conversations c
INNER JOIN public.agent_llm al
    ON al.agent_id = c.agent_id
INNER JOIN public.providers p
    ON p.id = al.provider_id
WHERE c.id = :conversation_id::UUID;
