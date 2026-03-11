// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OrgBalance {
    pub balance_microcents: i64,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct OrgBalanceQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<OrgBalance, tokio_postgres::Error>,
    mapper: fn(OrgBalance) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> OrgBalanceQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(OrgBalance) -> R) -> OrgBalanceQuery<'c, 'a, 's, C, R, N> {
        OrgBalanceQuery {
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
pub struct GetOrgBalanceStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_org_balance() -> GetOrgBalanceStmt {
    GetOrgBalanceStmt(
        "SELECT balance_microcents FROM org.orgs WHERE id = public.b64url_to_uuid($1::TEXT)",
        None,
    )
}
impl GetOrgBalanceStmt {
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
    ) -> OrgBalanceQuery<'c, 'a, 's, C, OrgBalance, 1> {
        OrgBalanceQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<OrgBalance, tokio_postgres::Error> {
                Ok(OrgBalance {
                    balance_microcents: row.try_get(0)?,
                })
            },
            mapper: |it| OrgBalance::from(it),
        }
    }
}
