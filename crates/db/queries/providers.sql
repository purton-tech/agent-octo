--: ResolvedProviderConfig()

--! get_provider_for_conversation : ResolvedProviderConfig
WITH target_conversation AS (
    SELECT
        c.id,
        c.org_id,
        a.default_connection_id,
        a.default_model
    FROM public.conversations c
    LEFT JOIN public.agents a
        ON a.id = c.agent_id
    WHERE c.id = :conversation_id::UUID
),
resolved_connection AS (
    SELECT
        pc.id,
        pc.provider_kind,
        pc.api_key,
        pc.base_url
    FROM target_conversation tc
    INNER JOIN LATERAL (
        SELECT
            c.id,
            c.provider_kind,
            c.api_key,
            c.base_url
        FROM public.provider_connections c
        WHERE c.id = tc.default_connection_id
           OR (
                tc.default_connection_id IS NULL
                AND c.org_id = tc.org_id
           )
        ORDER BY
            CASE
                WHEN c.id = tc.default_connection_id THEN 0
                ELSE 1
            END,
            c.created_at ASC
        LIMIT 1
    ) pc
        ON TRUE
),
resolved_model AS (
    SELECT
        COALESCE(
            tc.default_model,
            (
                SELECT pm.model
                FROM public.provider_models pm
                INNER JOIN resolved_connection rc
                    ON rc.id = pm.connection_id
                WHERE pm.is_enabled = TRUE
                ORDER BY pm.created_at ASC
                LIMIT 1
            )
        ) AS model
    FROM target_conversation tc
)
SELECT
    rc.id AS connection_id,
    rc.provider_kind,
    rc.api_key,
    COALESCE(rc.base_url, '') AS base_url,
    COALESCE(rm.model, '') AS model
FROM resolved_connection rc
INNER JOIN resolved_model rm
    ON TRUE;
