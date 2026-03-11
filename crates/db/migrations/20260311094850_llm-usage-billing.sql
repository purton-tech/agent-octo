-- migrate:up
-- =========================
-- LLM USAGE BILLING
-- =========================

CREATE TABLE public.llm_usage_events (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES org.orgs(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL REFERENCES public.conversations(id) ON DELETE CASCADE,
    input_tokens INTEGER NOT NULL,
    output_tokens INTEGER NOT NULL,
    cost_microcents BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.llm_usage_events IS
'Immutable ledger of LLM usage charges recorded against an organization.';

COMMENT ON COLUMN public.llm_usage_events.org_id IS
'Organization charged for the usage event.';

COMMENT ON COLUMN public.llm_usage_events.conversation_id IS
'Conversation that produced the model usage.';

COMMENT ON COLUMN public.llm_usage_events.input_tokens IS
'Prompt/input tokens used for the model call.';

COMMENT ON COLUMN public.llm_usage_events.output_tokens IS
'Completion/output tokens used for the model call.';

COMMENT ON COLUMN public.llm_usage_events.cost_microcents IS
'Charged cost for this usage event in microcents.';

COMMENT ON COLUMN public.llm_usage_events.created_at IS
'Timestamp when the usage event was recorded.';

CREATE INDEX llm_usage_events_org_created_at_idx
    ON public.llm_usage_events (org_id, created_at DESC);

CREATE INDEX llm_usage_events_conversation_created_at_idx
    ON public.llm_usage_events (conversation_id, created_at DESC);

GRANT SELECT, INSERT ON public.llm_usage_events TO application_user;
GRANT SELECT ON public.llm_usage_events TO application_readonly;

-- migrate:down
DROP TABLE IF EXISTS public.llm_usage_events;
