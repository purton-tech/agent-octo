// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateProviderConnectionParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
> {
    pub org_id: T1,
    pub provider_kind: T2,
    pub display_name: T3,
    pub api_key: T4,
    pub base_url: Option<T5>,
    pub default_model: Option<T6>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProviderConnectionCard {
    pub id: uuid::Uuid,
    pub provider_kind: String,
    pub display_name: String,
    pub base_url: String,
    pub default_model: String,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ProviderConnectionCardBorrowed<'a> {
    pub id: uuid::Uuid,
    pub provider_kind: &'a str,
    pub display_name: &'a str,
    pub base_url: &'a str,
    pub default_model: &'a str,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<ProviderConnectionCardBorrowed<'a>> for ProviderConnectionCard {
    fn from(
        ProviderConnectionCardBorrowed {
            id,
            provider_kind,
            display_name,
            base_url,
            default_model,
            updated_at,
        }: ProviderConnectionCardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            provider_kind: provider_kind.into(),
            display_name: display_name.into(),
            base_url: base_url.into(),
            default_model: default_model.into(),
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ProviderConnectionSetup {
    pub configured: bool,
}
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
pub struct ProviderConnectionCardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ProviderConnectionCardBorrowed, tokio_postgres::Error>,
    mapper: fn(ProviderConnectionCardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProviderConnectionCardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProviderConnectionCardBorrowed) -> R,
    ) -> ProviderConnectionCardQuery<'c, 'a, 's, C, R, N> {
        ProviderConnectionCardQuery {
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
pub struct ProviderConnectionSetupQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ProviderConnectionSetup, tokio_postgres::Error>,
    mapper: fn(ProviderConnectionSetup) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ProviderConnectionSetupQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ProviderConnectionSetup) -> R,
    ) -> ProviderConnectionSetupQuery<'c, 'a, 's, C, R, N> {
        ProviderConnectionSetupQuery {
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
pub struct ListProviderConnectionsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_provider_connections() -> ListProviderConnectionsStmt {
    ListProviderConnectionsStmt(
        "SELECT id, provider_kind, display_name, COALESCE(base_url, '') AS base_url, COALESCE(default_model, '') AS default_model, updated_at FROM public.provider_connections WHERE org_id = public.b64url_to_uuid($1::TEXT) ORDER BY updated_at DESC",
        None,
    )
}
impl ListProviderConnectionsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        org_id: &'a T1,
    ) -> ProviderConnectionCardQuery<'c, 'a, 's, C, ProviderConnectionCard, 1> {
        ProviderConnectionCardQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ProviderConnectionCardBorrowed, tokio_postgres::Error> {
                Ok(ProviderConnectionCardBorrowed {
                    id: row.try_get(0)?,
                    provider_kind: row.try_get(1)?,
                    display_name: row.try_get(2)?,
                    base_url: row.try_get(3)?,
                    default_model: row.try_get(4)?,
                    updated_at: row.try_get(5)?,
                })
            },
            mapper: |it| ProviderConnectionCard::from(it),
        }
    }
}
pub struct CreateProviderConnectionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_provider_connection() -> CreateProviderConnectionStmt {
    CreateProviderConnectionStmt(
        "WITH inserted AS ( INSERT INTO public.provider_connections ( org_id, created_by_user_id, provider_kind, display_name, api_key, base_url, default_model ) VALUES ( public.b64url_to_uuid($1::TEXT), auth.uid(), $2::TEXT, $3::TEXT, $4::TEXT, $5::TEXT, $6::TEXT ) RETURNING 1 ) SELECT EXISTS(SELECT 1 FROM inserted) AS configured",
        None,
    )
}
impl CreateProviderConnectionStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        org_id: &'a T1,
        provider_kind: &'a T2,
        display_name: &'a T3,
        api_key: &'a T4,
        base_url: &'a Option<T5>,
        default_model: &'a Option<T6>,
    ) -> ProviderConnectionSetupQuery<'c, 'a, 's, C, ProviderConnectionSetup, 6> {
        ProviderConnectionSetupQuery {
            client,
            params: [
                org_id,
                provider_kind,
                display_name,
                api_key,
                base_url,
                default_model,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ProviderConnectionSetup, tokio_postgres::Error> {
                Ok(ProviderConnectionSetup {
                    configured: row.try_get(0)?,
                })
            },
            mapper: |it| ProviderConnectionSetup::from(it),
        }
    }
}
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateProviderConnectionParams<T1, T2, T3, T4, T5, T6>,
        ProviderConnectionSetupQuery<'c, 'a, 's, C, ProviderConnectionSetup, 6>,
        C,
    > for CreateProviderConnectionStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateProviderConnectionParams<T1, T2, T3, T4, T5, T6>,
    ) -> ProviderConnectionSetupQuery<'c, 'a, 's, C, ProviderConnectionSetup, 6> {
        self.bind(
            client,
            &params.org_id,
            &params.provider_kind,
            &params.display_name,
            &params.api_key,
            &params.base_url,
            &params.default_model,
        )
    }
}
pub struct GetProviderForConversationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_provider_for_conversation() -> GetProviderForConversationStmt {
    GetProviderForConversationStmt(
        "WITH target_conversation AS ( SELECT c.id, c.org_id, a.default_connection_id, a.default_model FROM public.conversations c LEFT JOIN public.agents a ON a.id = c.agent_id WHERE c.id = $1::UUID ), resolved_connection AS ( SELECT pc.id, pc.provider_kind, pc.api_key, pc.base_url, pc.default_model FROM target_conversation tc INNER JOIN LATERAL ( SELECT c.id, c.provider_kind, c.api_key, c.base_url, c.default_model FROM public.provider_connections c WHERE c.id = tc.default_connection_id OR ( tc.default_connection_id IS NULL AND c.org_id = tc.org_id ) ORDER BY CASE WHEN c.id = tc.default_connection_id THEN 0 ELSE 1 END, c.created_at ASC LIMIT 1 ) pc ON TRUE ), resolved_model AS ( SELECT COALESCE( tc.default_model, rc.default_model, ( SELECT pm.model FROM public.provider_models pm INNER JOIN resolved_connection rc2 ON rc2.id = pm.connection_id WHERE pm.is_enabled = TRUE ORDER BY pm.created_at ASC LIMIT 1 ) ) AS model FROM target_conversation tc INNER JOIN resolved_connection rc ON TRUE ) SELECT rc.id AS connection_id, rc.provider_kind, rc.api_key, COALESCE(rc.base_url, '') AS base_url, COALESCE(rm.model, '') AS model FROM resolved_connection rc INNER JOIN resolved_model rm ON TRUE",
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
