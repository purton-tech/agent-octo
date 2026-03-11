--: UsageCharge()

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
