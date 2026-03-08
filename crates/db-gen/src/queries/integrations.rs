// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetIntegrationForEditParams<T1: crate::StringSql> {
    pub id: uuid::Uuid,
    pub org_id: T1,
}
#[derive(Debug)]
pub struct CreateIntegrationParams<T1: crate::StringSql, T2: crate::JsonSql> {
    pub org_id: T1,
    pub visibility: crate::types::ResourceVisibility,
    pub openapi_spec: T2,
}
#[derive(Debug)]
pub struct UpdateIntegrationParams<T1: crate::JsonSql, T2: crate::StringSql> {
    pub visibility: crate::types::ResourceVisibility,
    pub openapi_spec: T1,
    pub id: uuid::Uuid,
    pub org_id: T2,
}
#[derive(Debug)]
pub struct DeleteIntegrationParams<T1: crate::StringSql> {
    pub id: uuid::Uuid,
    pub org_id: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationCard {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub visibility: String,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct IntegrationCardBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub visibility: &'a str,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<IntegrationCardBorrowed<'a>> for IntegrationCard {
    fn from(
        IntegrationCardBorrowed {
            id,
            name,
            description,
            visibility,
            updated_at,
        }: IntegrationCardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            visibility: visibility.into(),
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationForm {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub visibility: String,
    pub openapi_spec: String,
}
pub struct IntegrationFormBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub visibility: &'a str,
    pub openapi_spec: &'a str,
}
impl<'a> From<IntegrationFormBorrowed<'a>> for IntegrationForm {
    fn from(
        IntegrationFormBorrowed {
            id,
            name,
            description,
            visibility,
            openapi_spec,
        }: IntegrationFormBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            visibility: visibility.into(),
            openapi_spec: openapi_spec.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IntegrationMutation {
    pub changed: bool,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct IntegrationCardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<IntegrationCardBorrowed, tokio_postgres::Error>,
    mapper: fn(IntegrationCardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> IntegrationCardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(IntegrationCardBorrowed) -> R,
    ) -> IntegrationCardQuery<'c, 'a, 's, C, R, N> {
        IntegrationCardQuery {
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
pub struct IntegrationFormQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<IntegrationFormBorrowed, tokio_postgres::Error>,
    mapper: fn(IntegrationFormBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> IntegrationFormQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(IntegrationFormBorrowed) -> R,
    ) -> IntegrationFormQuery<'c, 'a, 's, C, R, N> {
        IntegrationFormQuery {
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
pub struct IntegrationMutationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<IntegrationMutation, tokio_postgres::Error>,
    mapper: fn(IntegrationMutation) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> IntegrationMutationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(IntegrationMutation) -> R,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, R, N> {
        IntegrationMutationQuery {
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
pub struct ListIntegrationsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_integrations() -> ListIntegrationsStmt {
    ListIntegrationsStmt(
        "SELECT i.id, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name, COALESCE(i.openapi_spec #>> '{info,description}', '') AS description, i.visibility::TEXT AS visibility, i.updated_at FROM public.integrations i WHERE i.org_id = public.b64url_to_uuid($1::TEXT) AND ( i.visibility = 'org' OR i.created_by_user_id = auth.uid() ) ORDER BY i.updated_at DESC",
        None,
    )
}
impl ListIntegrationsStmt {
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
    ) -> IntegrationCardQuery<'c, 'a, 's, C, IntegrationCard, 1> {
        IntegrationCardQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<IntegrationCardBorrowed, tokio_postgres::Error> {
                Ok(IntegrationCardBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    visibility: row.try_get(3)?,
                    updated_at: row.try_get(4)?,
                })
            },
            mapper: |it| IntegrationCard::from(it),
        }
    }
}
pub struct GetIntegrationForEditStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_integration_for_edit() -> GetIntegrationForEditStmt {
    GetIntegrationForEditStmt(
        "SELECT i.id, COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name, COALESCE(i.openapi_spec #>> '{info,description}', '') AS description, i.visibility::TEXT AS visibility, i.openapi_spec::TEXT AS openapi_spec FROM public.integrations i WHERE i.id = $1::UUID AND i.org_id = public.b64url_to_uuid($2::TEXT) LIMIT 1",
        None,
    )
}
impl GetIntegrationForEditStmt {
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
        id: &'a uuid::Uuid,
        org_id: &'a T1,
    ) -> IntegrationFormQuery<'c, 'a, 's, C, IntegrationForm, 2> {
        IntegrationFormQuery {
            client,
            params: [id, org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<IntegrationFormBorrowed, tokio_postgres::Error> {
                Ok(IntegrationFormBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    visibility: row.try_get(3)?,
                    openapi_spec: row.try_get(4)?,
                })
            },
            mapper: |it| IntegrationForm::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetIntegrationForEditParams<T1>,
        IntegrationFormQuery<'c, 'a, 's, C, IntegrationForm, 2>,
        C,
    > for GetIntegrationForEditStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetIntegrationForEditParams<T1>,
    ) -> IntegrationFormQuery<'c, 'a, 's, C, IntegrationForm, 2> {
        self.bind(client, &params.id, &params.org_id)
    }
}
pub struct CreateIntegrationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_integration() -> CreateIntegrationStmt {
    CreateIntegrationStmt(
        "WITH inserted AS ( INSERT INTO public.integrations ( org_id, created_by_user_id, visibility, openapi_spec ) VALUES ( public.b64url_to_uuid($1::TEXT), auth.uid(), $2::resource_visibility, $3::JSONB ) RETURNING id ) SELECT EXISTS(SELECT 1 FROM inserted) AS changed",
        None,
    )
}
impl CreateIntegrationStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::JsonSql>(
        &'s self,
        client: &'c C,
        org_id: &'a T1,
        visibility: &'a crate::types::ResourceVisibility,
        openapi_spec: &'a T2,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 3> {
        IntegrationMutationQuery {
            client,
            params: [org_id, visibility, openapi_spec],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<IntegrationMutation, tokio_postgres::Error> {
                    Ok(IntegrationMutation {
                        changed: row.try_get(0)?,
                    })
                },
            mapper: |it| IntegrationMutation::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::JsonSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateIntegrationParams<T1, T2>,
        IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 3>,
        C,
    > for CreateIntegrationStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateIntegrationParams<T1, T2>,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 3> {
        self.bind(
            client,
            &params.org_id,
            &params.visibility,
            &params.openapi_spec,
        )
    }
}
pub struct UpdateIntegrationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_integration() -> UpdateIntegrationStmt {
    UpdateIntegrationStmt(
        "WITH updated AS ( UPDATE public.integrations i SET visibility = $1::resource_visibility, openapi_spec = $2::JSONB, updated_at = NOW() WHERE i.id = $3::UUID AND i.org_id = public.b64url_to_uuid($4::TEXT) RETURNING id ) SELECT EXISTS(SELECT 1 FROM updated) AS changed",
        None,
    )
}
impl UpdateIntegrationStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::JsonSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        visibility: &'a crate::types::ResourceVisibility,
        openapi_spec: &'a T1,
        id: &'a uuid::Uuid,
        org_id: &'a T2,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 4> {
        IntegrationMutationQuery {
            client,
            params: [visibility, openapi_spec, id, org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<IntegrationMutation, tokio_postgres::Error> {
                    Ok(IntegrationMutation {
                        changed: row.try_get(0)?,
                    })
                },
            mapper: |it| IntegrationMutation::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::JsonSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateIntegrationParams<T1, T2>,
        IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 4>,
        C,
    > for UpdateIntegrationStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateIntegrationParams<T1, T2>,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 4> {
        self.bind(
            client,
            &params.visibility,
            &params.openapi_spec,
            &params.id,
            &params.org_id,
        )
    }
}
pub struct DeleteIntegrationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_integration() -> DeleteIntegrationStmt {
    DeleteIntegrationStmt(
        "WITH deleted AS ( DELETE FROM public.integrations i WHERE i.id = $1::UUID AND i.org_id = public.b64url_to_uuid($2::TEXT) RETURNING id ) SELECT EXISTS(SELECT 1 FROM deleted) AS changed",
        None,
    )
}
impl DeleteIntegrationStmt {
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
        id: &'a uuid::Uuid,
        org_id: &'a T1,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 2> {
        IntegrationMutationQuery {
            client,
            params: [id, org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<IntegrationMutation, tokio_postgres::Error> {
                    Ok(IntegrationMutation {
                        changed: row.try_get(0)?,
                    })
                },
            mapper: |it| IntegrationMutation::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        DeleteIntegrationParams<T1>,
        IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 2>,
        C,
    > for DeleteIntegrationStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a DeleteIntegrationParams<T1>,
    ) -> IntegrationMutationQuery<'c, 'a, 's, C, IntegrationMutation, 2> {
        self.bind(client, &params.id, &params.org_id)
    }
}
