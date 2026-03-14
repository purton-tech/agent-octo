--: UsageCharge()
--: StripeCustomerRecord()
--: TopUpTransaction()
--: TopUpTransactionId()
--: TopUpCompletion()
--: TopUpTransactionStatus()

--! record_llm_usage_for_conversation : UsageCharge
WITH billing_context AS (
    SELECT
        c.org_id,
        p.price_per_million_input_microcents AS input_price_microcents,
        p.price_per_million_output_microcents AS output_price_microcents
    FROM public.conversations c
    INNER JOIN public.agent_llm al
        ON al.agent_id = c.agent_id
    INNER JOIN public.providers p
        ON p.id = al.provider_id
    WHERE c.id = :conversation_id::UUID
),
computed_charge AS (
    SELECT
        org_id,
        (
            (:input_tokens::BIGINT * input_price_microcents) / 1000000
        ) + (
            (:output_tokens::BIGINT * output_price_microcents) / 1000000
        ) AS cost_microcents
    FROM billing_context
),
inserted_usage AS (
    INSERT INTO public.llm_usage_events (
        org_id,
        conversation_id,
        input_tokens,
        output_tokens,
        cost_microcents
    )
    SELECT
        cc.org_id,
        :conversation_id::UUID,
        :input_tokens::INT,
        :output_tokens::INT,
        cc.cost_microcents
    FROM computed_charge cc
    RETURNING
        id,
        org_id,
        conversation_id,
        input_tokens,
        output_tokens,
        cost_microcents,
        created_at
),
updated_org AS (
    UPDATE org.orgs o
    SET balance_microcents = o.balance_microcents - iu.cost_microcents
    FROM inserted_usage iu
    WHERE o.id = iu.org_id
    RETURNING o.balance_microcents
)
SELECT
    iu.id,
    iu.org_id,
    iu.conversation_id,
    iu.input_tokens,
    iu.output_tokens,
    iu.cost_microcents,
    iu.created_at
FROM inserted_usage iu
INNER JOIN updated_org uo
    ON TRUE;

--! get_stripe_customer_for_org : StripeCustomerRecord
SELECT
    stripe_customer_id
FROM billing.stripe_customers
WHERE org_id = public.b64url_to_uuid(:org_id::TEXT);

--! upsert_stripe_customer_for_org : StripeCustomerRecord
INSERT INTO billing.stripe_customers (
    org_id,
    stripe_customer_id
)
VALUES (
    public.b64url_to_uuid(:org_id::TEXT),
    :stripe_customer_id::TEXT
)
ON CONFLICT (org_id)
DO UPDATE SET
    stripe_customer_id = EXCLUDED.stripe_customer_id,
    updated_at = NOW()
RETURNING
    stripe_customer_id;

--! create_top_up_transaction : TopUpTransactionId
INSERT INTO billing.top_up_transactions (
    org_id,
    user_id,
    amount_microcents,
    status
)
VALUES (
    public.b64url_to_uuid(:org_id::TEXT),
    :user_id::UUID,
    :amount_microcents::BIGINT,
    'pending'
)
RETURNING
    id;

--! attach_top_up_checkout_session
UPDATE billing.top_up_transactions
SET stripe_checkout_session_id = :stripe_checkout_session_id::TEXT
WHERE id = :transaction_id::UUID;

--! mark_top_up_transaction_failed
UPDATE billing.top_up_transactions
SET status = 'failed'
WHERE id = :transaction_id::UUID
  AND status = 'pending';

--! list_top_up_transactions : TopUpTransaction
SELECT
    amount_microcents,
    status,
    created_at,
    COALESCE(completed_at, created_at) AS completed_at
FROM billing.top_up_transactions
WHERE org_id = public.b64url_to_uuid(:org_id::TEXT)
ORDER BY created_at DESC;

--! get_top_up_transaction_for_org : TopUpTransactionStatus
SELECT
    status,
    COALESCE(stripe_checkout_session_id, '') AS stripe_checkout_session_id
FROM billing.top_up_transactions
WHERE id = :transaction_id::UUID
  AND org_id = public.b64url_to_uuid(:org_id::TEXT);

--! complete_top_up_checkout_session (stripe_payment_intent?) : TopUpCompletion
WITH updated_transaction AS (
    UPDATE billing.top_up_transactions
    SET status = 'succeeded',
        stripe_payment_intent_id = COALESCE(
            :stripe_payment_intent::TEXT,
            stripe_payment_intent_id
        ),
        stripe_event_id = :stripe_event_id::TEXT,
        completed_at = NOW()
    WHERE stripe_checkout_session_id = :stripe_checkout_session_id::TEXT
      AND status = 'pending'
    RETURNING org_id, amount_microcents
),
updated_org AS (
    UPDATE org.orgs o
    SET balance_microcents = o.balance_microcents + ut.amount_microcents
    FROM updated_transaction ut
    WHERE o.id = ut.org_id
    RETURNING ut.org_id, ut.amount_microcents
)
SELECT
    EXISTS(SELECT 1 FROM updated_org) AS applied,
    COALESCE(
        (
            SELECT public.uuid_to_b64url(org_id)
            FROM updated_org
            LIMIT 1
        ),
        ''
    ) AS org_public_id,
    COALESCE(
        (
            SELECT amount_microcents
            FROM updated_org
            LIMIT 1
        ),
        0
    ) AS amount_microcents;
