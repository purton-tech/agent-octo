-- migrate:up
-- =========================
-- INTEGRATIONS (OPENAPI) + CONNECTIONS (AUTH)
-- =========================
--
-- integrations:
--   OpenAPI specifications that define toolable HTTP APIs.
--   These are "capabilities/templates" and can be private or org-shared.
--
-- integration_connections:
--   Per-org configured credentials for an integration (API keys / OAuth2).
--   These are usually per-user (private) but can be org-shared (e.g. sales inbox).
--   Secrets are referenced, never stored.
--
-- RLS:
--   - integrations: org members can read org-visible + their own private; creator manages private; admins manage org-visible
--   - integration_connections: org members can read org-visible + their own private; creator manages private; admins manage org-visible
--
-- Agents will later link to integration_connections (separate migration).

CREATE TYPE integration_auth_type AS ENUM (
    'api_key',
    'oauth2'
);

COMMENT ON TYPE integration_auth_type IS
'Authentication mechanism used by an integration connection. api_key covers static keys; oauth2 covers access/refresh token flows.';

-- =========================
-- INTEGRATIONS
-- =========================

CREATE TABLE public.integrations (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    visibility resource_visibility NOT NULL DEFAULT 'private',

    name TEXT NOT NULL,
    description TEXT,

    openapi_spec JSONB NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.integrations IS
'OpenAPI-based integrations that can be turned into tool definitions. Scoped to an org; can be private or shared to org.';

COMMENT ON COLUMN public.integrations.visibility IS
'private = only creator can see/use; org = visible to org members (RLS enforced).';

COMMENT ON COLUMN public.integrations.openapi_spec IS
'OpenAPI specification JSON used to generate tools (endpoints/operations).';

CREATE INDEX integrations_org_visibility_idx
    ON public.integrations (org_id, visibility);

CREATE INDEX integrations_creator_idx
    ON public.integrations (created_by_user_id);

-- =========================
-- INTEGRATION CONNECTIONS (CREDENTIALS)
-- =========================

CREATE TABLE public.integration_connections (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    integration_id UUID NOT NULL REFERENCES public.integrations(id) ON DELETE CASCADE,

    created_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,

    visibility resource_visibility NOT NULL DEFAULT 'private',

    name TEXT NOT NULL,                           -- e.g. "GitHub - personal", "Gmail - work"
    auth_type integration_auth_type NOT NULL,

    -- API Key auth
    api_key_secret_ref TEXT,

    -- OAuth2 auth (store tokens in secret store; refresh optional)
    oauth_access_token_secret_ref TEXT,
    oauth_refresh_token_secret_ref TEXT,
    oauth_expires_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure auth fields match auth_type (minimal correctness)
    CONSTRAINT integration_connections_auth_fields_chk CHECK (
        (auth_type = 'api_key'
            AND api_key_secret_ref IS NOT NULL
            AND oauth_access_token_secret_ref IS NULL
            AND oauth_refresh_token_secret_ref IS NULL
        )
        OR
        (auth_type = 'oauth2'
            AND oauth_access_token_secret_ref IS NOT NULL
        )
    )
);

COMMENT ON TABLE public.integration_connections IS
'Per-org configured credentials for an integration. Usually private to a user; optionally shared with the org. Secrets are stored externally and referenced here.';

COMMENT ON COLUMN public.integration_connections.visibility IS
'private = only creator can see/use; org = visible to org members (RLS enforced).';

COMMENT ON COLUMN public.integration_connections.integration_id IS
'Which OpenAPI integration this connection configures.';

COMMENT ON COLUMN public.integration_connections.name IS
'Human-friendly name for this configured connection (often identifies which account).';

COMMENT ON COLUMN public.integration_connections.api_key_secret_ref IS
'Secret store reference for API key credentials (auth_type=api_key).';

COMMENT ON COLUMN public.integration_connections.oauth_access_token_secret_ref IS
'Secret store reference for OAuth2 access token (auth_type=oauth2).';

COMMENT ON COLUMN public.integration_connections.oauth_refresh_token_secret_ref IS
'Optional secret store reference for OAuth2 refresh token (auth_type=oauth2).';

COMMENT ON COLUMN public.integration_connections.oauth_expires_at IS
'Optional access token expiry timestamp (auth_type=oauth2).';

CREATE INDEX integration_connections_org_visibility_idx
    ON public.integration_connections (org_id, visibility);

CREATE INDEX integration_connections_integration_idx
    ON public.integration_connections (integration_id);

-- =========================
-- RLS
-- =========================

ALTER TABLE public.integrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.integration_connections ENABLE ROW LEVEL SECURITY;

-- ---- integrations ----

CREATE POLICY integrations_select
ON public.integrations
FOR SELECT
USING (
    org.is_org_member(org_id)
    AND (
        visibility = 'org'
        OR created_by_user_id = auth.uid()
    )
);

CREATE POLICY integrations_insert
ON public.integrations
FOR INSERT
WITH CHECK (
    org.is_org_member(org_id)
    AND created_by_user_id = auth.uid()
);

CREATE POLICY integrations_update
ON public.integrations
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

CREATE POLICY integrations_delete
ON public.integrations
FOR DELETE
USING (
    org.is_org_member(org_id)
    AND (
        (visibility = 'private' AND created_by_user_id = auth.uid())
        OR (visibility = 'org' AND org.is_org_admin(org_id))
    )
);

-- ---- integration_connections ----

CREATE POLICY integration_connections_select
ON public.integration_connections
FOR SELECT
USING (
    org.is_org_member(org_id)
    AND (
        visibility = 'org'
        OR created_by_user_id = auth.uid()
    )
);

CREATE POLICY integration_connections_insert
ON public.integration_connections
FOR INSERT
WITH CHECK (
    org.is_org_member(org_id)
    AND created_by_user_id = auth.uid()
);

CREATE POLICY integration_connections_update
ON public.integration_connections
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

CREATE POLICY integration_connections_delete
ON public.integration_connections
FOR DELETE
USING (
    org.is_org_member(org_id)
    AND (
        (visibility = 'private' AND created_by_user_id = auth.uid())
        OR (visibility = 'org' AND org.is_org_admin(org_id))
    )
);

-- migrate:down
DROP POLICY IF EXISTS integration_connections_delete ON public.integration_connections;
DROP POLICY IF EXISTS integration_connections_update ON public.integration_connections;
DROP POLICY IF EXISTS integration_connections_insert ON public.integration_connections;
DROP POLICY IF EXISTS integration_connections_select ON public.integration_connections;

DROP POLICY IF EXISTS integrations_delete ON public.integrations;
DROP POLICY IF EXISTS integrations_update ON public.integrations;
DROP POLICY IF EXISTS integrations_insert ON public.integrations;
DROP POLICY IF EXISTS integrations_select ON public.integrations;

DROP TABLE IF EXISTS public.integration_connections;
DROP TABLE IF EXISTS public.integrations;

DROP TYPE IF EXISTS integration_auth_type;
