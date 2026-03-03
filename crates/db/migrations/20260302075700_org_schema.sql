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
DROP INDEX IF EXISTS org_memberships_one_owner_per_org_idx;
DROP TABLE IF EXISTS org_memberships;
DROP TABLE IF EXISTS orgs;
DROP TYPE IF EXISTS org_role;
