// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct EnsureDefaultAgent {
    pub inserted: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AgentCard {
    pub id: uuid::Uuid,
    pub name: String,
    pub visibility: String,
    pub description: String,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct AgentCardBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub visibility: &'a str,
    pub description: &'a str,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<AgentCardBorrowed<'a>> for AgentCard {
    fn from(
        AgentCardBorrowed {
            id,
            name,
            visibility,
            description,
            updated_at,
        }: AgentCardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            visibility: visibility.into(),
            description: description.into(),
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct EnsureDefaultAgentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<EnsureDefaultAgent, tokio_postgres::Error>,
    mapper: fn(EnsureDefaultAgent) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> EnsureDefaultAgentQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(EnsureDefaultAgent) -> R,
    ) -> EnsureDefaultAgentQuery<'c, 'a, 's, C, R, N> {
        EnsureDefaultAgentQuery {
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
pub struct AgentCardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<AgentCardBorrowed, tokio_postgres::Error>,
    mapper: fn(AgentCardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AgentCardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AgentCardBorrowed) -> R) -> AgentCardQuery<'c, 'a, 's, C, R, N> {
        AgentCardQuery {
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
pub struct EnsureDefaultAgentForUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn ensure_default_agent_for_user() -> EnsureDefaultAgentForUserStmt {
    EnsureDefaultAgentForUserStmt(
        "WITH inserted AS ( INSERT INTO public.agents ( org_id, created_by_user_id, visibility, name, description, system_prompt ) SELECT $1::UUID, auth.uid(), 'private'::resource_visibility, 'My First Agent', 'Your default assistant.', 'You are a helpful assistant.' WHERE auth.uid() IS NOT NULL AND org.is_org_member($1::UUID) AND NOT EXISTS ( SELECT 1 FROM public.agents a WHERE a.org_id = $1::UUID AND a.created_by_user_id = auth.uid() ) RETURNING 1 ) SELECT EXISTS(SELECT 1 FROM inserted) AS inserted",
        None,
    )
}
impl EnsureDefaultAgentForUserStmt {
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
        org_id: &'a uuid::Uuid,
    ) -> EnsureDefaultAgentQuery<'c, 'a, 's, C, EnsureDefaultAgent, 1> {
        EnsureDefaultAgentQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<EnsureDefaultAgent, tokio_postgres::Error> {
                    Ok(EnsureDefaultAgent {
                        inserted: row.try_get(0)?,
                    })
                },
            mapper: |it| EnsureDefaultAgent::from(it),
        }
    }
}
pub struct ListMyAgentsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_my_agents() -> ListMyAgentsStmt {
    ListMyAgentsStmt(
        "SELECT id, name, visibility::TEXT AS visibility, COALESCE(description, '') AS description, updated_at FROM public.agents WHERE created_by_user_id = auth.uid() AND org_id = public.b64url_to_uuid($1::TEXT) AND ( visibility = 'org' OR created_by_user_id = auth.uid() ) ORDER BY updated_at DESC",
        None,
    )
}
impl ListMyAgentsStmt {
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
    ) -> AgentCardQuery<'c, 'a, 's, C, AgentCard, 1> {
        AgentCardQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<AgentCardBorrowed, tokio_postgres::Error> {
                    Ok(AgentCardBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        visibility: row.try_get(2)?,
                        description: row.try_get(3)?,
                        updated_at: row.try_get(4)?,
                    })
                },
            mapper: |it| AgentCard::from(it),
        }
    }
}
