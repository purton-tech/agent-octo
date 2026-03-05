--: AgentCard()

--! list_my_agents : AgentCard
SELECT
    id,
    name,
    visibility::TEXT AS visibility,
    COALESCE(description, '') AS description,
    updated_at
FROM public.agents
WHERE created_by_user_id = auth.uid()
  AND org_id = public.b64url_to_uuid(:org_id::TEXT)
ORDER BY updated_at DESC;
