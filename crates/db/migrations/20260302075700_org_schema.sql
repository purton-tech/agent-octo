-- migrate:up
CREATE TYPE org_role AS ENUM (
    'owner',
    'admin',
    'member'
);

CREATE TABLE orgs (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE org_memberships (
    org_id UUID NOT NULL REFERENCES orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    role org_role NOT NULL,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (org_id, user_id)
);

CREATE UNIQUE INDEX org_memberships_one_owner_per_org_idx
    ON org_memberships (org_id)
    WHERE role = 'owner';

CREATE FUNCTION enforce_org_has_owner_from_membership()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
    target_org_id UUID;
BEGIN
    target_org_id := COALESCE(NEW.org_id, OLD.org_id);

    IF NOT EXISTS (
        SELECT 1
        FROM orgs
        WHERE id = target_org_id
    ) THEN
        RETURN NULL;
    END IF;

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

CREATE FUNCTION enforce_org_has_owner_from_org()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
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

CREATE CONSTRAINT TRIGGER org_memberships_require_owner
AFTER INSERT OR UPDATE OR DELETE ON org_memberships
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION enforce_org_has_owner_from_membership();

CREATE CONSTRAINT TRIGGER orgs_require_owner
AFTER INSERT ON orgs
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION enforce_org_has_owner_from_org();

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
