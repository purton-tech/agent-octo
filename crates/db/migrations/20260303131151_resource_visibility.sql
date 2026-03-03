-- migrate:up
-- =========================
-- RESOURCE VISIBILITY
-- =========================
--
-- Controls whether a resource is:
--   - private: visible only to the creator
--   - org: visible to all members of the organization
--
-- Used by:
--   - agents
--   - integrations
--   - integration_connections
--   - channels
--   - any other org-scoped resources

CREATE TYPE resource_visibility AS ENUM (
    'private',
    'org'
);

COMMENT ON TYPE resource_visibility IS
'Controls resource sharing scope. private = creator only; org = visible to all org members (RLS enforced).';

-- migrate:down
DROP TYPE IF EXISTS resource_visibility;