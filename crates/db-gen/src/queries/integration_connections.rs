// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetIntegrationAuthRequirementParams<T1: crate::StringSql> {
    pub integration_id: uuid::Uuid,
    pub org_id: T1,
}
#[derive(Debug)]
pub struct CreateIntegrationConnectionParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub org_id: T1,
    pub integration_id: uuid::Uuid,
    pub visibility: crate::types::ResourceVisibility,
    pub api_key_secret_ref: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationConnectionCard {
    pub id: uuid::Uuid,
    pub integration_id: uuid::Uuid,
    pub name: String,
    pub integration_name: String,
    pub auth_type: String,
    pub visibility: String,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct IntegrationConnectionCardBorrowed<'a> {
    pub id: uuid::Uuid,
    pub integration_id: uuid::Uuid,
    pub name: &'a str,
    pub integration_name: &'a str,
    pub auth_type: &'a str,
    pub visibility: &'a str,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<IntegrationConnectionCardBorrowed<'a>> for IntegrationConnectionCard {
    fn from(
        IntegrationConnectionCardBorrowed {
            id,
            integration_id,
            name,
            integration_name,
            auth_type,
            visibility,
            updated_at,
        }: IntegrationConnectionCardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            integration_id,
            name: name.into(),
            integration_name: integration_name.into(),
            auth_type: auth_type.into(),
            visibility: visibility.into(),
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectableIntegration {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub requires_auth: bool,
}
pub struct ConnectableIntegrationBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub requires_auth: bool,
}
impl<'a> From<ConnectableIntegrationBorrowed<'a>> for ConnectableIntegration {
    fn from(
        ConnectableIntegrationBorrowed {
            id,
            name,
            description,
            requires_auth,
        }: ConnectableIntegrationBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            requires_auth,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IntegrationAuthRequirement {
    pub integration_id: uuid::Uuid,
    pub requires_auth: bool,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IntegrationConnectionMutation {
    pub changed: bool,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct IntegrationConnectionCardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(
        &tokio_postgres::Row,
    ) -> Result<IntegrationConnectionCardBorrowed, tokio_postgres::Error>,
    mapper: fn(IntegrationConnectionCardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> IntegrationConnectionCardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(IntegrationConnectionCardBorrowed) -> R,
    ) -> IntegrationConnectionCardQuery<'c, 'a, 's, C, R, N> {
        IntegrationConnectionCardQuery {
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
pub struct ConnectableIntegrationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ConnectableIntegrationBorrowed, tokio_postgres::Error>,
    mapper: fn(ConnectableIntegrationBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ConnectableIntegrationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ConnectableIntegrationBorrowed) -> R,
    ) -> ConnectableIntegrationQuery<'c, 'a, 's, C, R, N> {
        ConnectableIntegrationQuery {
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
pub struct IntegrationAuthRequirementQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<IntegrationAuthRequirement, tokio_postgres::Error>,
    mapper: fn(IntegrationAuthRequirement) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> IntegrationAuthRequirementQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(IntegrationAuthRequirement) -> R,
    ) -> IntegrationAuthRequirementQuery<'c, 'a, 's, C, R, N> {
        IntegrationAuthRequirementQuery {
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
pub struct IntegrationConnectionMutationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<IntegrationConnectionMutation, tokio_postgres::Error>,
    mapper: fn(IntegrationConnectionMutation) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> IntegrationConnectionMutationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(IntegrationConnectionMutation) -> R,
    ) -> IntegrationConnectionMutationQuery<'c, 'a, 's, C, R, N> {
        IntegrationConnectionMutationQuery {
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
pub struct ListIntegrationConnectionsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_integration_connections() -> ListIntegrationConnectionsStmt {
    ListIntegrationConnectionsStmt(
        "SELECT ic.id, ic.integration_id, ic.name, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS integration_name, ic.auth_type::TEXT AS auth_type, ic.visibility::TEXT AS visibility, ic.updated_at FROM public.integration_connections ic JOIN public.integrations i ON i.id = ic.integration_id WHERE ic.org_id = public.b64url_to_uuid($1::TEXT) AND ( ic.visibility = 'org' OR ic.created_by_user_id = auth.uid() ) ORDER BY ic.updated_at DESC",
        None,
    )
}
impl ListIntegrationConnectionsStmt {
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
    ) -> IntegrationConnectionCardQuery<'c, 'a, 's, C, IntegrationConnectionCard, 1> {
        IntegrationConnectionCardQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<IntegrationConnectionCardBorrowed, tokio_postgres::Error> {
                Ok(IntegrationConnectionCardBorrowed {
                    id: row.try_get(0)?,
                    integration_id: row.try_get(1)?,
                    name: row.try_get(2)?,
                    integration_name: row.try_get(3)?,
                    auth_type: row.try_get(4)?,
                    visibility: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| IntegrationConnectionCard::from(it),
        }
    }
}
pub struct ListConnectableIntegrationsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_connectable_integrations() -> ListConnectableIntegrationsStmt {
    ListConnectableIntegrationsStmt(
        "SELECT i.id, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name, COALESCE(i.openapi_spec #>> '{info,description}', '') AS description, ( jsonb_path_exists(i.openapi_spec, '$.security[*]') OR jsonb_path_exists(i.openapi_spec, '$.components.securitySchemes.*') ) AS requires_auth FROM public.integrations i WHERE i.org_id = public.b64url_to_uuid($1::TEXT) AND ( i.visibility = 'org' OR i.created_by_user_id = auth.uid() ) ORDER BY name ASC",
        None,
    )
}
impl ListConnectableIntegrationsStmt {
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
    ) -> ConnectableIntegrationQuery<'c, 'a, 's, C, ConnectableIntegration, 1> {
        ConnectableIntegrationQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ConnectableIntegrationBorrowed, tokio_postgres::Error> {
                Ok(ConnectableIntegrationBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    requires_auth: row.try_get(3)?,
                })
            },
            mapper: |it| ConnectableIntegration::from(it),
        }
    }
}
pub struct GetIntegrationAuthRequirementStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_integration_auth_requirement() -> GetIntegrationAuthRequirementStmt {
    GetIntegrationAuthRequirementStmt(
        "SELECT i.id AS integration_id, ( jsonb_path_exists(i.openapi_spec, '$.security[*]') OR jsonb_path_exists(i.openapi_spec, '$.components.securitySchemes.*') ) AS requires_auth FROM public.integrations i WHERE i.id = $1::UUID AND i.org_id = public.b64url_to_uuid($2::TEXT) LIMIT 1",
        None,
    )
}
impl GetIntegrationAuthRequirementStmt {
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
        integration_id: &'a uuid::Uuid,
        org_id: &'a T1,
    ) -> IntegrationAuthRequirementQuery<'c, 'a, 's, C, IntegrationAuthRequirement, 2> {
        IntegrationAuthRequirementQuery {
            client,
            params: [integration_id, org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<IntegrationAuthRequirement, tokio_postgres::Error> {
                Ok(IntegrationAuthRequirement {
                    integration_id: row.try_get(0)?,
                    requires_auth: row.try_get(1)?,
                })
            },
            mapper: |it| IntegrationAuthRequirement::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetIntegrationAuthRequirementParams<T1>,
        IntegrationAuthRequirementQuery<'c, 'a, 's, C, IntegrationAuthRequirement, 2>,
        C,
    > for GetIntegrationAuthRequirementStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetIntegrationAuthRequirementParams<T1>,
    ) -> IntegrationAuthRequirementQuery<'c, 'a, 's, C, IntegrationAuthRequirement, 2> {
        self.bind(client, &params.integration_id, &params.org_id)
    }
}
pub struct CreateIntegrationConnectionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_integration_connection() -> CreateIntegrationConnectionStmt {
    CreateIntegrationConnectionStmt(
        "WITH inserted AS ( INSERT INTO public.integration_connections ( org_id, integration_id, created_by_user_id, visibility, name, auth_type, api_key_secret_ref ) SELECT public.b64url_to_uuid($1::TEXT), $2::UUID, auth.uid(), $3::resource_visibility, COALESCE(i.openapi_spec #>> '{info,title}', 'Integration') || ' connection', 'api_key'::integration_auth_type, $4::TEXT FROM public.integrations i WHERE i.id = $2::UUID AND i.org_id = public.b64url_to_uuid($1::TEXT) RETURNING id ) SELECT EXISTS(SELECT 1 FROM inserted) AS changed",
        None,
    )
}
impl CreateIntegrationConnectionStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        org_id: &'a T1,
        integration_id: &'a uuid::Uuid,
        visibility: &'a crate::types::ResourceVisibility,
        api_key_secret_ref: &'a T2,
    ) -> IntegrationConnectionMutationQuery<'c, 'a, 's, C, IntegrationConnectionMutation, 4> {
        IntegrationConnectionMutationQuery {
            client,
            params: [org_id, integration_id, visibility, api_key_secret_ref],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<IntegrationConnectionMutation, tokio_postgres::Error> {
                Ok(IntegrationConnectionMutation {
                    changed: row.try_get(0)?,
                })
            },
            mapper: |it| IntegrationConnectionMutation::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateIntegrationConnectionParams<T1, T2>,
        IntegrationConnectionMutationQuery<'c, 'a, 's, C, IntegrationConnectionMutation, 4>,
        C,
    > for CreateIntegrationConnectionStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateIntegrationConnectionParams<T1, T2>,
    ) -> IntegrationConnectionMutationQuery<'c, 'a, 's, C, IntegrationConnectionMutation, 4> {
        self.bind(
            client,
            &params.org_id,
            &params.integration_id,
            &params.visibility,
            &params.api_key_secret_ref,
        )
    }
}
