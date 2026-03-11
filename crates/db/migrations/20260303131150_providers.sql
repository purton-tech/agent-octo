-- migrate:up
-- =========================
-- PROVIDERS + AGENT LLM
-- =========================
--
-- providers:
--   Global provider catalog with required defaults.
--
-- agent_llm:
--   Per-agent provider + credentials + optional model override.

GRANT USAGE ON SCHEMA public TO application_user;
GRANT USAGE ON SCHEMA public TO application_readonly;

CREATE TABLE public.providers (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    svg_logo TEXT NOT NULL,

    default_model_name TEXT NOT NULL,
    default_model_display_name TEXT NOT NULL,
    default_model_context_size INT NOT NULL DEFAULT 0,
    default_model_description TEXT NOT NULL DEFAULT '',
    price_per_million_input_microcents BIGINT NOT NULL DEFAULT 3000000,
    price_per_million_output_microcents BIGINT NOT NULL DEFAULT 3000000,

    base_url TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.providers IS
'Global LLM provider catalog with required default model metadata.';

COMMENT ON COLUMN public.providers.price_per_million_input_microcents IS
'Internal input token pricing in microcents per 1,000,000 tokens.';

COMMENT ON COLUMN public.providers.price_per_million_output_microcents IS
'Internal output token pricing in microcents per 1,000,000 tokens.';

CREATE TABLE public.agent_llm (
    agent_id UUID PRIMARY KEY,
    provider_id UUID NOT NULL REFERENCES public.providers(id),
    api_key TEXT NOT NULL,
    model_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE public.agent_llm IS
'Per-agent LLM configuration including provider, api_key and optional model override.';

CREATE INDEX agent_llm_provider_idx
    ON public.agent_llm (provider_id);

-- Seed catalog providers with mandatory defaults.
INSERT INTO public.providers (
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url
)
VALUES
    (
        'openai',
        'openai.svg',
        'gpt-4o-mini',
        'GPT-4o mini',
        128000,
        'Balanced cost/performance default for OpenAI.',
        'https://api.openai.com/v1'
    ),
    (
        'anthropic',
        'anthropic.svg',
        'claude-3-5-sonnet-latest',
        'Claude 3.5 Sonnet',
        200000,
        'General-purpose Claude default.',
        'https://api.anthropic.com/v1'
    ),
    (
        'gemini',
        'gemini.svg',
        'gemini-2.0-flash',
        'Gemini 2.0 Flash',
        1048576,
        'Fast default Gemini model.',
        'https://generativelanguage.googleapis.com/v1beta/openai'
    );

GRANT SELECT ON public.providers TO application_user;
GRANT SELECT ON public.providers TO application_readonly;
GRANT SELECT, INSERT, UPDATE, DELETE ON public.agent_llm TO application_user;
GRANT SELECT ON public.agent_llm TO application_readonly;

-- migrate:down
DROP TABLE IF EXISTS public.agent_llm;
DROP TABLE IF EXISTS public.providers;
