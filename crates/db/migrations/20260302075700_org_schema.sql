-- migrate:up
-- =========================
-- ORG
-- =========================
--
-- Multi-tenant organization model.
--
-- Concepts:
--   - orgs: tenant/accounts
--   - org_memberships: users belong to orgs with a role
--   - org_invitations: invitation flow for adding members
--
-- Invariants:
--   - Each org must always have an owner
--   - At most one owner per org (enforced by partial unique index)
--   - At least one owner per org (enforced by deferred constraint triggers)
--
-- Notes:
--   - The "must have an owner" checks are DEFERRABLE INITIALLY DEFERRED so
--     org creation can happen in one transaction:
--       INSERT orgs ...
--       INSERT org_memberships ... role='owner'
--     and the constraint is checked at COMMIT.
--
-- UUID generation:
--   - uuidv7() assumed available (Postgres 18 / uuidv7 extension), producing
--     time-ordered UUIDs.

CREATE TYPE org_role AS ENUM (
    'owner',
    'admin',
    'member'
);

COMMENT ON TYPE org_role IS
'Role of a user within an organization. owner is the top-level role; admin can manage most org resources; member is standard access.';

CREATE TABLE orgs (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE orgs IS
'Organizations (tenants/accounts). Most product data should be scoped to an org_id referencing this table.';

COMMENT ON COLUMN orgs.id IS
'Primary key for the organization (UUIDv7).';

COMMENT ON COLUMN orgs.name IS
'Human-friendly organization name.';

COMMENT ON COLUMN orgs.created_at IS
'Creation timestamp.';

CREATE TABLE org_memberships (
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    role org_role NOT NULL,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (org_id, user_id)
);

COMMENT ON TABLE org_memberships IS
'Join table mapping users to organizations with a role. Used for tenant isolation and authorization.';

COMMENT ON COLUMN org_memberships.org_id IS
'Organization the user belongs to.';

COMMENT ON COLUMN org_memberships.user_id IS
'User who is a member of the organization.';

COMMENT ON COLUMN org_memberships.role IS
'Role of the user within the organization.';

COMMENT ON COLUMN org_memberships.joined_at IS
'Timestamp when the user joined the organization.';

-- At most one owner per org.
CREATE UNIQUE INDEX org_memberships_one_owner_per_org_idx
    ON org_memberships (org_id)
    WHERE role = 'owner';

COMMENT ON INDEX org_memberships_one_owner_per_org_idx IS
'Enforces that an organization has at most one owner (partial unique index where role = owner).';

-- =========================
-- OWNER INVARIANT ENFORCEMENT
-- =========================
--
-- We enforce "org must have an owner" using deferred constraint triggers.
-- This guarantees that after any transaction, every org has at least one owner,
-- while still allowing multi-step transactions.

CREATE FUNCTION enforce_org_has_owner_from_membership()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
    target_org_id UUID;
BEGIN
    -- Determine which org to validate (INSERT/UPDATE uses NEW, DELETE uses OLD).
    target_org_id := COALESCE(NEW.org_id, OLD.org_id);

    -- If the org row no longer exists (e.g. cascading delete), do nothing.
    IF NOT EXISTS (
        SELECT 1
        FROM orgs
        WHERE id = target_org_id
    ) THEN
        RETURN NULL;
    END IF;

    -- Enforce: at least one owner exists for this org.
    -- ("At most one" is handled by the partial unique index.)
    IF NOT EXISTS (
        SELECT 1
        FROM org_memberships
        WHERE org_id = target_org_id
          AND role = 'owner'
    ) THEN
        RAISE EXCEPTION 'org % must have exactly one owner', target_org_id;
    END IF;

    RETURN NULL;
END;
$$;

COMMENT ON FUNCTION enforce_org_has_owner_from_membership() IS
'Deferred constraint trigger function. Ensures every org has an owner after membership changes.';

CREATE FUNCTION enforce_org_has_owner_from_org()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    -- Enforce: when creating an org, there must be an owner membership by commit.
    IF NOT EXISTS (
        SELECT 1
        FROM org_memberships
        WHERE org_id = NEW.id
          AND role = 'owner'
    ) THEN
        RAISE EXCEPTION 'org % must have exactly one owner', NEW.id;
    END IF;

    RETURN NULL;
END;
$$;

COMMENT ON FUNCTION enforce_org_has_owner_from_org() IS
'Deferred constraint trigger function. Ensures org creation is accompanied by an owner membership before commit.';

CREATE CONSTRAINT TRIGGER org_memberships_require_owner
AFTER INSERT OR UPDATE OR DELETE ON org_memberships
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION enforce_org_has_owner_from_membership();

COMMENT ON TRIGGER org_memberships_require_owner ON org_memberships IS
'Deferred constraint trigger enforcing that each org has an owner after membership mutations.';

CREATE CONSTRAINT TRIGGER orgs_require_owner
AFTER INSERT ON orgs
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION enforce_org_has_owner_from_org();

COMMENT ON TRIGGER orgs_require_owner ON orgs IS
'Deferred constraint trigger enforcing that new orgs have an owner membership before commit.';

-- =========================
-- INVITATIONS
-- =========================
--
-- Invitation records support adding users to orgs.
-- - invited_by_user_id: who created the invite (must be a user)
-- - accepted_by_user_id: set when the invite is accepted (optional)
-- - expires_at: optional expiration timestamp

CREATE TABLE org_invitations (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    email VARCHAR NOT NULL,
    role org_role NOT NULL,
    invited_by_user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    accepted_by_user_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (org_id, email)
);

COMMENT ON TABLE org_invitations IS
'Invitations to join an organization. Uniqueness is per (org_id, email) to avoid duplicate invites.';

COMMENT ON COLUMN org_invitations.id IS
'Primary key for the invitation (UUIDv7).';

COMMENT ON COLUMN org_invitations.org_id IS
'Organization the invite applies to.';

COMMENT ON COLUMN org_invitations.email IS
'Email address the invite is sent to (identity resolution happens at acceptance time).';

COMMENT ON COLUMN org_invitations.role IS
'Role to grant the invited user upon acceptance.';

COMMENT ON COLUMN org_invitations.invited_by_user_id IS
'User who created the invitation.';

COMMENT ON COLUMN org_invitations.accepted_by_user_id IS
'User who accepted the invitation (nullable until accepted).';

COMMENT ON COLUMN org_invitations.expires_at IS
'Optional expiration time after which the invite should be considered invalid.';

COMMENT ON COLUMN org_invitations.created_at IS
'Creation timestamp.';

-- migrate:down
DROP TABLE IF EXISTS org_invitations;
DROP TRIGGER IF EXISTS orgs_require_owner ON orgs;
DROP TRIGGER IF EXISTS org_memberships_require_owner ON org_memberships;
DROP FUNCTION IF EXISTS enforce_org_has_owner_from_org();
DROP FUNCTION IF EXISTS enforce_org_has_owner_from_membership();
DROP INDEX IF EXISTS org_memberships_one_owner_per_org_idx;
DROP TABLE IF EXISTS org_memberships;
DROP TABLE IF EXISTS orgs;
DROP TYPE IF EXISTS org_role;