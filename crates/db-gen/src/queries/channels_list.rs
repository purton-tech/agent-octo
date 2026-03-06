// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct ConnectTelegramChannelParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub org_id: T1,
    pub bot_token: T2,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChannelSetup {
    pub configured: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelCard {
    pub id: uuid::Uuid,
    pub name: String,
    pub kind: String,
    pub visibility: String,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ChannelCardBorrowed<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub kind: &'a str,
    pub visibility: &'a str,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<ChannelCardBorrowed<'a>> for ChannelCard {
    fn from(
        ChannelCardBorrowed {
            id,
            name,
            kind,
            visibility,
            updated_at,
        }: ChannelCardBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            kind: kind.into(),
            visibility: visibility.into(),
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ChannelSetupQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ChannelSetup, tokio_postgres::Error>,
    mapper: fn(ChannelSetup) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ChannelSetupQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ChannelSetup) -> R) -> ChannelSetupQuery<'c, 'a, 's, C, R, N> {
        ChannelSetupQuery {
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
pub struct ChannelCardQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ChannelCardBorrowed, tokio_postgres::Error>,
    mapper: fn(ChannelCardBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ChannelCardQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ChannelCardBorrowed) -> R,
    ) -> ChannelCardQuery<'c, 'a, 's, C, R, N> {
        ChannelCardQuery {
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
pub struct HasTelegramChannelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn has_telegram_channel() -> HasTelegramChannelStmt {
    HasTelegramChannelStmt(
        "SELECT EXISTS ( SELECT 1 FROM public.channels c WHERE c.org_id = public.b64url_to_uuid($1::TEXT) AND c.kind = 'telegram'::channel_type ) AS configured",
        None,
    )
}
impl HasTelegramChannelStmt {
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
    ) -> ChannelSetupQuery<'c, 'a, 's, C, ChannelSetup, 1> {
        ChannelSetupQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<ChannelSetup, tokio_postgres::Error> {
                Ok(ChannelSetup {
                    configured: row.try_get(0)?,
                })
            },
            mapper: |it| ChannelSetup::from(it),
        }
    }
}
pub struct ConnectTelegramChannelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn connect_telegram_channel() -> ConnectTelegramChannelStmt {
    ConnectTelegramChannelStmt(
        "WITH inserted AS ( INSERT INTO public.channels ( org_id, created_by_user_id, visibility, kind, name, bot_token ) SELECT public.b64url_to_uuid($1::TEXT), auth.uid(), 'private'::resource_visibility, 'telegram'::channel_type, 'Telegram', $2::TEXT WHERE NOT EXISTS ( SELECT 1 FROM public.channels c WHERE c.org_id = public.b64url_to_uuid($1::TEXT) AND c.kind = 'telegram'::channel_type ) RETURNING 1 ) SELECT EXISTS(SELECT 1 FROM inserted) AS configured",
        None,
    )
}
impl ConnectTelegramChannelStmt {
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
        bot_token: &'a T2,
    ) -> ChannelSetupQuery<'c, 'a, 's, C, ChannelSetup, 2> {
        ChannelSetupQuery {
            client,
            params: [org_id, bot_token],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<ChannelSetup, tokio_postgres::Error> {
                Ok(ChannelSetup {
                    configured: row.try_get(0)?,
                })
            },
            mapper: |it| ChannelSetup::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ConnectTelegramChannelParams<T1, T2>,
        ChannelSetupQuery<'c, 'a, 's, C, ChannelSetup, 2>,
        C,
    > for ConnectTelegramChannelStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ConnectTelegramChannelParams<T1, T2>,
    ) -> ChannelSetupQuery<'c, 'a, 's, C, ChannelSetup, 2> {
        self.bind(client, &params.org_id, &params.bot_token)
    }
}
pub struct ListOrgChannelsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_org_channels() -> ListOrgChannelsStmt {
    ListOrgChannelsStmt(
        "SELECT id, name, kind::TEXT AS kind, visibility::TEXT AS visibility, updated_at FROM public.channels WHERE org_id = public.b64url_to_uuid($1::TEXT) AND ( visibility = 'org' OR created_by_user_id = auth.uid() ) ORDER BY updated_at DESC",
        None,
    )
}
impl ListOrgChannelsStmt {
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
    ) -> ChannelCardQuery<'c, 'a, 's, C, ChannelCard, 1> {
        ChannelCardQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ChannelCardBorrowed, tokio_postgres::Error> {
                    Ok(ChannelCardBorrowed {
                        id: row.try_get(0)?,
                        name: row.try_get(1)?,
                        kind: row.try_get(2)?,
                        visibility: row.try_get(3)?,
                        updated_at: row.try_get(4)?,
                    })
                },
            mapper: |it| ChannelCard::from(it),
        }
    }
}
