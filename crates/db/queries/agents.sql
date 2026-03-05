--: AgentCard()
--: EnsureDefaultAgent()

--! ensure_default_agent_for_user : EnsureDefaultAgent
WITH inserted AS (
    INSERT INTO public.agents (
        org_id,
        created_by_user_id,
        visibility,
        name,
        description,
        system_prompt
    )
    SELECT
        :org_id::UUID,
        :user_id::UUID,
        'private'::resource_visibility,
        'My First Agent',
        'Your default assistant.',
        'You are a helpful assistant.'
    WHERE NOT EXISTS (
        SELECT 1
        FROM public.agents a
        WHERE a.org_id = :org_id::UUID
          AND a.created_by_user_id = :user_id::UUID
    )
    RETURNING 1
)
SELECT EXISTS(SELECT 1 FROM inserted) AS inserted;

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
  AND (
      visibility = 'org'
      OR created_by_user_id = auth.uid()
  )
ORDER BY updated_at DESC;
