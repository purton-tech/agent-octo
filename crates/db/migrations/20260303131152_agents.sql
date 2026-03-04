-- migrate:up
-- =========================
-- AGENTS
-- =========================
--
-- Agents are configurable tool-using assistants.
--
-- Key ideas:
--   - Scoped to an org (tenant isolation).
--   - Created by a user.
--   - Private by default; can be shared to org.
--   - Optionally pinned to a provider connection + model.
--
-- Note:
--   - Secrets are never stored here.
--   - Tool/plugin wiring is handled via join tables (separate migration).

CREATE TABLE public.agents (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    visibility resource_visibility NOT NULL DEFAULT 'private',

    name TEXT NOT NULL,
    description TEXT,
    system_prompt TEXT NOT NULL,

    default_connection_id UUID REFERENCES public.provider_connections(id) ON DELETE SET NULL,
    default_model TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.agents IS
'Configurable assistants (agents) scoped to an org. Private by default; can be shared with the org via visibility=org.';

COMMENT ON COLUMN public.agents.visibility IS
'private = only creator can see/use; org = visible to all org members (RLS enforced).';

COMMENT ON COLUMN public.agents.system_prompt IS
'System prompt defining the agent behavior and instructions.';

COMMENT ON COLUMN public.agents.default_connection_id IS
'Optional default provider connection for this agent.';

COMMENT ON COLUMN public.agents.default_model IS
'Optional default model identifier (provider-specific string).';

CREATE INDEX agents_org_visibility_idx
    ON public.agents (org_id, visibility);

CREATE INDEX agents_creator_idx
    ON public.agents (created_by_user_id);

GRANT SELECT, INSERT, UPDATE, DELETE ON public.agents TO application_user;
GRANT SELECT ON public.agents TO application_readonly;

-- =========================
-- RLS
-- =========================

ALTER TABLE public.agents ENABLE ROW LEVEL SECURITY;

-- Read: org members can see org-visible agents, plus their own private agents.
CREATE POLICY agents_select
ON public.agents
FOR SELECT
USING (
    org.is_org_member(org_id)
    AND (
        visibility = 'org'
        OR created_by_user_id = auth.uid()
    )
);

-- Insert: must be in your org, and you must be the creator.
CREATE POLICY agents_insert
ON public.agents
FOR INSERT
WITH CHECK (
    org.is_org_member(org_id)
    AND created_by_user_id = auth.uid()
);

-- Update:
--   - Creator can update their private agent.
--   - Org admins can update org-visible agents.
CREATE POLICY agents_update
ON public.agents
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
--   - Creator can delete their private agent.
--   - Org admins can delete org-visible agents.
CREATE POLICY agents_delete
ON public.agents
FOR DELETE
USING (
    org.is_org_member(org_id)
    AND (
        (visibility = 'private' AND created_by_user_id = auth.uid())
        OR (visibility = 'org' AND org.is_org_admin(org_id))
    )
);

-- migrate:down
DROP POLICY IF EXISTS agents_delete ON public.agents;
DROP POLICY IF EXISTS agents_update ON public.agents;
DROP POLICY IF EXISTS agents_insert ON public.agents;
DROP POLICY IF EXISTS agents_select ON public.agents;

DROP TABLE IF EXISTS public.agents;
