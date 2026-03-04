#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../.." && pwd)"
env_file="${1:-${repo_root}/.env}"

if [[ ! -f "${env_file}" ]]; then
    echo "Missing env file: ${env_file}" >&2
    exit 1
fi

if ! command -v psql >/dev/null 2>&1; then
    echo "psql is required but not installed" >&2
    exit 1
fi

set -a
. "${env_file}"
set +a

: "${APPLICATION_URL:?APPLICATION_URL not set in ${env_file}}"
: "${TELEGRAM_BOT_TOKEN:?TELEGRAM_BOT_TOKEN not set in ${env_file}}"

psql "${APPLICATION_URL}" \
    --set=ON_ERROR_STOP=1 \
    --set=telegram_bot_token="${TELEGRAM_BOT_TOKEN}" <<'SQL'
BEGIN;

INSERT INTO auth.users (
    issuer,
    sub,
    email,
    first_name,
    last_name
)
VALUES (
    'dev-setup',
    'dev-setup-user',
    'dev-setup@example.com',
    'Dev',
    'Setup'
)
RETURNING id
\gset bootstrap_user_

INSERT INTO org.orgs (name)
VALUES ('Dev Setup Org')
RETURNING id
\gset bootstrap_org_

INSERT INTO org.org_memberships (
    org_id,
    user_id,
    role
)
VALUES (
    :'bootstrap_org_id',
    :'bootstrap_user_id',
    'owner'
);

SET LOCAL request.jwt.claim.sub = :'bootstrap_user_id';

INSERT INTO public.agents (
    org_id,
    created_by_user_id,
    visibility,
    name,
    description,
    system_prompt
)
VALUES (
    :'bootstrap_org_id',
    :'bootstrap_user_id',
    'private',
    'Dev Setup Agent',
    'Bootstrap agent for local Telegram development.',
    'You are a helpful assistant for local development.'
)
RETURNING id
\gset bootstrap_agent_

INSERT INTO public.channels (
    org_id,
    created_by_user_id,
    visibility,
    kind,
    name,
    default_agent_id,
    bot_token
)
VALUES (
    :'bootstrap_org_id',
    :'bootstrap_user_id',
    'private',
    'telegram',
    'Dev Setup Telegram',
    :'bootstrap_agent_id',
    :'telegram_bot_token'
)
RETURNING id
\gset bootstrap_channel_

COMMIT;

\echo Created bootstrap records:
\echo user_id=:bootstrap_user_id
\echo org_id=:bootstrap_org_id
\echo agent_id=:bootstrap_agent_id
\echo channel_id=:bootstrap_channel_id
\echo channel_name=Dev Setup Telegram
SQL
