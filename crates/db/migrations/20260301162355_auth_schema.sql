-- migrate:up
CREATE SCHEMA IF NOT EXISTS auth;

GRANT USAGE ON SCHEMA auth TO application_user;
GRANT USAGE ON SCHEMA auth TO application_readonly;

-- =========================
-- AUTH USERS
-- =========================
--
-- Stores authenticated principals from external OIDC providers.
--
-- Identity is defined by (issuer, sub):
--   issuer = OIDC issuer URL (JWT "iss" claim)
--   sub    = stable subject identifier from that issuer (JWT "sub" claim)
--
-- Email is an attribute, not the primary identity key.
-- Users authenticate externally; this table represents them internally.

CREATE TABLE auth.users (
    id uuid PRIMARY KEY DEFAULT uuidv7(),

    issuer TEXT NOT NULL,
    sub TEXT NOT NULL,

    email TEXT NOT NULL,
    first_name TEXT,
    last_name TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (issuer, sub),
    UNIQUE (email)
);

COMMENT ON TABLE auth.users IS
'Authenticated principals derived from external OIDC providers. 
Canonical identity is (issuer, sub).';

COMMENT ON COLUMN auth.users.issuer IS
'OIDC issuer URL (JWT "iss" claim). Uniquely identifies the identity provider.';

COMMENT ON COLUMN auth.users.sub IS
'OIDC subject identifier (JWT "sub" claim). Stable unique user ID within the issuer.';

COMMENT ON COLUMN auth.users.email IS
'User email address as provided by the identity provider. Not the canonical identity key.';

-- =========================
-- AUTH CONTEXT (JWT CLAIMS)
-- =========================
--
-- This function mirrors the Supabase/PostgREST convention.
--
-- The API layer is responsible for:
--   1) Validating the JWT (signature, issuer, expiry, etc.)
--   2) Injecting JWT claims into the DB session using:
--        SET LOCAL request.jwt.claim.<claim> = 'value';
--
-- Postgres does NOT validate the JWT itself.
-- It only reads claims that have already been verified upstream.
--
-- auth.uid() returns the authenticated user's UUID
-- from the JWT "sub" claim.
--
-- If the claim is missing, NULL is returned.

CREATE FUNCTION auth.uid()
RETURNS uuid
LANGUAGE sql
STABLE
AS $$
  SELECT current_setting('request.jwt.claim.sub', true)::uuid
$$;

COMMENT ON FUNCTION auth.uid() IS
'Returns the authenticated user ID (JWT "sub" claim) from the current session. 
Requires the API layer to validate the JWT and inject claims via request.jwt.claim.* settings.';

GRANT SELECT, INSERT, UPDATE, DELETE ON auth.users TO application_user;
GRANT SELECT ON auth.users TO application_readonly;

GRANT EXECUTE ON FUNCTION auth.uid() TO application_user;
GRANT EXECUTE ON FUNCTION auth.uid() TO application_readonly;

-- migrate:down
DROP FUNCTION IF EXISTS auth.uid();
DROP TABLE IF EXISTS auth.users;
DROP SCHEMA IF EXISTS auth;
