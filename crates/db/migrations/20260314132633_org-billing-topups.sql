-- migrate:up
CREATE SCHEMA IF NOT EXISTS billing;

GRANT USAGE ON SCHEMA billing TO application_user;
GRANT USAGE ON SCHEMA billing TO application_readonly;

CREATE TABLE billing.stripe_customers (
    org_id UUID PRIMARY KEY REFERENCES org.orgs(id) ON DELETE CASCADE,
    stripe_customer_id TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE billing.stripe_customers IS
'Maps organizations to Stripe customer ids for checkout and future billing flows.';

CREATE TABLE billing.top_up_transactions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    amount_microcents BIGINT NOT NULL CHECK (amount_microcents > 0),
    status TEXT NOT NULL CHECK (status IN ('pending', 'succeeded', 'failed')),
    stripe_checkout_session_id TEXT UNIQUE,
    stripe_payment_intent_id TEXT UNIQUE,
    stripe_event_id TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

COMMENT ON TABLE billing.top_up_transactions IS
'Tracks prepaid balance top-up attempts and their reconciliation with Stripe.';

COMMENT ON COLUMN billing.top_up_transactions.amount_microcents IS
'Requested top-up amount in microcents.';

GRANT SELECT, INSERT, UPDATE, DELETE ON billing.stripe_customers TO application_user;
GRANT SELECT ON billing.stripe_customers TO application_readonly;
GRANT SELECT, INSERT, UPDATE, DELETE ON billing.top_up_transactions TO application_user;
GRANT SELECT ON billing.top_up_transactions TO application_readonly;

CREATE INDEX top_up_transactions_org_created_at_idx
    ON billing.top_up_transactions (org_id, created_at DESC);

-- migrate:down
DROP TABLE IF EXISTS billing.top_up_transactions;
DROP TABLE IF EXISTS billing.stripe_customers;
DROP SCHEMA IF EXISTS billing;
