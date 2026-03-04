// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedProviderConfig {
    pub connection_id: uuid::Uuid,
    pub provider_kind: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}
pub struct ResolvedProviderConfigBorrowed<'a> {
    pub connection_id: uuid::Uuid,
    pub provider_kind: &'a str,
    pub api_key: &'a str,
    pub base_url: &'a str,
    pub model: &'a str,
}
impl<'a> From<ResolvedProviderConfigBorrowed<'a>> for ResolvedProviderConfig {
    fn from(
        ResolvedProviderConfigBorrowed {
            connection_id,
            provider_kind,
            api_key,
            base_url,
            model,
        }: ResolvedProviderConfigBorrowed<'a>,
    ) -> Self {
        Self {
            connection_id,
            provider_kind: provider_kind.into(),
            api_key: api_key.into(),
            base_url: base_url.into(),
            model: model.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ResolvedProviderConfigQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ResolvedProviderConfigBorrowed, tokio_postgres::Error>,
    mapper: fn(ResolvedProviderConfigBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ResolvedProviderConfigQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ResolvedProviderConfigBorrowed) -> R,
    ) -> ResolvedProviderConfigQuery<'c, 'a, 's, C, R, N> {
        ResolvedProviderConfigQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetProviderForConversationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_provider_for_conversation() -> GetProviderForConversationStmt {
    GetProviderForConversationStmt(
        "WITH target_conversation AS ( SELECT c.id, c.org_id, a.default_connection_id, a.default_model FROM public.conversations c LEFT JOIN public.agents a ON a.id = c.agent_id WHERE c.id = $1::UUID ), resolved_connection AS ( SELECT pc.id, pc.provider_kind, pc.api_key, pc.base_url FROM target_conversation tc INNER JOIN LATERAL ( SELECT c.id, c.provider_kind, c.api_key, c.base_url FROM public.provider_connections c WHERE c.id = tc.default_connection_id OR ( tc.default_connection_id IS NULL AND c.org_id = tc.org_id ) ORDER BY CASE WHEN c.id = tc.default_connection_id THEN 0 ELSE 1 END, c.created_at ASC LIMIT 1 ) pc ON TRUE ), resolved_model AS ( SELECT COALESCE( tc.default_model, ( SELECT pm.model FROM public.provider_models pm INNER JOIN resolved_connection rc ON rc.id = pm.connection_id WHERE pm.is_enabled = TRUE ORDER BY pm.created_at ASC LIMIT 1 ) ) AS model FROM target_conversation tc ) SELECT rc.id AS connection_id, rc.provider_kind, rc.api_key, COALESCE(rc.base_url, '') AS base_url, COALESCE(rm.model, '') AS model FROM resolved_connection rc INNER JOIN resolved_model rm ON TRUE",
        None,
    )
}
impl GetProviderForConversationStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        conversation_id: &'a uuid::Uuid,
    ) -> ResolvedProviderConfigQuery<'c, 'a, 's, C, ResolvedProviderConfig, 1> {
        ResolvedProviderConfigQuery {
            client,
            params: [conversation_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ResolvedProviderConfigBorrowed, tokio_postgres::Error> {
                Ok(ResolvedProviderConfigBorrowed {
                    connection_id: row.try_get(0)?,
                    provider_kind: row.try_get(1)?,
                    api_key: row.try_get(2)?,
                    base_url: row.try_get(3)?,
                    model: row.try_get(4)?,
                })
            },
            mapper: |it| ResolvedProviderConfig::from(it),
        }
    }
}
