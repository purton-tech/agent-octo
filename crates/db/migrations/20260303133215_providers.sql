-- migrate:up
-- =========================
-- PROVIDERS + MODELS
-- =========================
--
-- provider_connections:
--   A configured LLM provider account for an org (API key stored in secret store).
--
-- provider_models:
--   Models available/allowed for a given connection. Populate manually or via sync.
--
-- Notes:
--   - Keep provider_kind/model as TEXT to avoid churn.
--   - Secrets are referenced, not stored.
--   - RLS: org members can read; org admins manage.

CREATE TABLE public.provider_connections (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    provider_kind TEXT NOT NULL,              -- "openai", "anthropic", "gemini", etc.
    display_name TEXT NOT NULL,

    api_key_secret_ref TEXT NOT NULL,         -- reference into your secret store
    base_url TEXT,                            -- optional (proxy/self-hosted/azure)

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.provider_connections IS
'Configured LLM provider connections (credentials/config) scoped to an org. Secrets are stored externally and referenced here.';

CREATE INDEX provider_connections_org_idx
    ON public.provider_connections (org_id);

CREATE TABLE public.provider_models (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    connection_id UUID NOT NULL REFERENCES public.provider_connections(id) ON DELETE CASCADE,

    model TEXT NOT NULL,                      -- e.g. "gpt-4o", "claude-3-5-sonnet"
    is_enabled BOOLEAN NOT NULL DEFAULT true,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (connection_id, model)
);

COMMENT ON TABLE public.provider_models IS
'Models available/allowed for a specific provider connection. Used to drive UI selection and enforce allow-lists.';

CREATE INDEX provider_models_connection_idx
    ON public.provider_models (connection_id);

-- =========================
-- RLS
-- =========================

ALTER TABLE public.provider_connections ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.provider_models ENABLE ROW LEVEL SECURITY;

-- provider_connections
CREATE POLICY provider_connections_select_member
ON public.provider_connections
FOR SELECT
USING (is_org_member(org_id));

CREATE POLICY provider_connections_insert_admin
ON public.provider_connections
FOR INSERT
WITH CHECK (
    is_org_admin(org_id)
    AND created_by_user_id = auth.uid()
);

CREATE POLICY provider_connections_update_admin
ON public.provider_connections
FOR UPDATE
USING (is_org_admin(org_id))
WITH CHECK (is_org_admin(org_id));

CREATE POLICY provider_connections_delete_admin
ON public.provider_connections
FOR DELETE
USING (is_org_admin(org_id));

-- provider_models: access derived from owning connection's org_id
CREATE POLICY provider_models_select_member
ON public.provider_models
FOR SELECT
USING (
    EXISTS (
        SELECT 1
        FROM public.provider_connections c
        WHERE c.id = connection_id
          AND is_org_member(c.org_id)
    )
);

CREATE POLICY provider_models_insert_admin
ON public.provider_models
FOR INSERT
WITH CHECK (
    EXISTS (
        SELECT 1
        FROM public.provider_connections c
        WHERE c.id = connection_id
          AND is_org_admin(c.org_id)
    )
);

CREATE POLICY provider_models_update_admin
ON public.provider_models
FOR UPDATE
USING (
    EXISTS (
        SELECT 1
        FROM public.provider_connections c
        WHERE c.id = connection_id
          AND is_org_admin(c.org_id)
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1
        FROM public.provider_connections c
        WHERE c.id = connection_id
          AND is_org_admin(c.org_id)
    )
);

CREATE POLICY provider_models_delete_admin
ON public.provider_models
FOR DELETE
USING (
    EXISTS (
        SELECT 1
        FROM public.provider_connections c
        WHERE c.id = connection_id
          AND is_org_admin(c.org_id)
    )
);

-- migrate:down
DROP POLICY IF EXISTS provider_models_delete_admin ON public.provider_models;
DROP POLICY IF EXISTS provider_models_update_admin ON public.provider_models;
DROP POLICY IF EXISTS provider_models_insert_admin ON public.provider_models;
DROP POLICY IF EXISTS provider_models_select_member ON public.provider_models;

DROP POLICY IF EXISTS provider_connections_delete_admin ON public.provider_connections;
DROP POLICY IF EXISTS provider_connections_update_admin ON public.provider_connections;
DROP POLICY IF EXISTS provider_connections_insert_admin ON public.provider_connections;
DROP POLICY IF EXISTS provider_connections_select_member ON public.provider_connections;

DROP TABLE IF EXISTS public.provider_models;
DROP TABLE IF EXISTS public.provider_connections;