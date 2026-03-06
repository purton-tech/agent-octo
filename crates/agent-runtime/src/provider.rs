use anyhow::{Context, anyhow};
use db::clorinde::queries::providers::{ResolvedProviderConfig, get_provider_for_conversation};
use rig::providers::openai::Client;

pub async fn load_for_conversation(
    client: &db::clorinde::deadpool_postgres::Client,
    conversation_id: &uuid::Uuid,
) -> anyhow::Result<ResolvedProviderConfig> {
    let provider = get_provider_for_conversation()
        .bind(client, conversation_id)
        .opt()
        .await
        .context("failed to load provider configuration")?
        .ok_or_else(|| anyhow!("no agent_llm configured for conversation"))?;

    if provider.model.is_empty() {
        return Err(anyhow!(
            "no model configured for provider {}: set agent_llm.model_name or provider default",
            provider.connection_id
        ));
    }

    Ok(provider)
}

pub fn build_client(provider: &ResolvedProviderConfig) -> anyhow::Result<Client> {
    match provider.provider_kind.as_str() {
        "openai" => {
            let mut builder = Client::builder().api_key(&provider.api_key);

            if !provider.base_url.is_empty() {
                builder = builder.base_url(&provider.base_url);
            }

            builder
                .build()
                .map_err(|err| anyhow!("failed to build OpenAI client: {err}"))
        }
        other => Err(anyhow!("unsupported provider kind: {other}")),
    }
}
