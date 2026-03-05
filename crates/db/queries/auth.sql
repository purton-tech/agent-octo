--: User()

--: AuthUser()
--: EnsureOrgMembership()
--: UserOrg()

--! upsert_user_by_issuer_sub (first_name?, last_name?) : AuthUser
INSERT INTO auth.users (
    issuer,
    sub,
    email,
    first_name,
    last_name
)
VALUES (
    :issuer::TEXT,
    :sub::TEXT,
    :email::TEXT,
    :first_name::TEXT,
    :last_name::TEXT
)
ON CONFLICT (issuer, sub)
DO UPDATE SET
    email = EXCLUDED.email,
    first_name = EXCLUDED.first_name,
    last_name = EXCLUDED.last_name,
    updated_at = NOW()
RETURNING
    id,
    issuer,
    sub,
    email;

--! get_current_user : AuthUser
SELECT
    id,
    issuer,
    sub,
    email
FROM auth.users
WHERE id = auth.uid();

--! ensure_default_org_membership_for_user : EnsureOrgMembership
WITH has_membership AS (
    SELECT 1
    FROM org.org_memberships
    WHERE user_id = :user_id::UUID
    LIMIT 1
),
inserted_org AS (
    INSERT INTO org.orgs (name)
    SELECT :org_name::TEXT
    WHERE NOT EXISTS (SELECT 1 FROM has_membership)
    RETURNING id
),
inserted_membership AS (
    INSERT INTO org.org_memberships (org_id, user_id, role)
    SELECT io.id, :user_id::UUID, 'owner'::org.org_role
    FROM inserted_org io
    ON CONFLICT (org_id, user_id) DO NOTHING
    RETURNING 1
)
SELECT EXISTS(SELECT 1 FROM inserted_membership) AS ensured;

--! get_first_org_for_user : UserOrg
SELECT
    org_id
FROM org.org_memberships
WHERE user_id = :user_id::UUID
ORDER BY joined_at ASC
LIMIT 1;

--! set_request_claim_sub
SELECT set_config(
    'request.jwt.claim.sub',
    :claim_sub::TEXT,
    true
);

--! set_request_claim_iss
SELECT set_config(
    'request.jwt.claim.iss',
    :claim_iss::TEXT,
    true
);

--! set_request_claim_external_sub
SELECT set_config(
    'request.jwt.claim.external_sub',
    :claim_external_sub::TEXT,
    true
);

--! get_users : User
SELECT 
    id, 
    email
FROM auth.users;
