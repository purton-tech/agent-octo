// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertChannelMessageParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::JsonSql,
> {
    pub channel: crate::types::ChannelType,
    pub direction: crate::types::ChannelMessageDirection,
    pub external_conversation_id: T1,
    pub external_user_id: Option<T2>,
    pub external_message_id: Option<T3>,
    pub message_text: T4,
    pub status: crate::types::ChannelMessageStatus,
    pub metadata_json: T5,
}
#[derive(Clone, Copy, Debug)]
pub struct UpdateChannelMessageStatusParams {
    pub status: crate::types::ChannelMessageStatus,
    pub id: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct ClaimNextChannelMessageParams {
    pub channel: crate::types::ChannelType,
    pub direction: crate::types::ChannelMessageDirection,
    pub from_status: crate::types::ChannelMessageStatus,
    pub to_status: crate::types::ChannelMessageStatus,
}
#[derive(Debug)]
pub struct ListConversationMessagesParams<T1: crate::StringSql> {
    pub channel: crate::types::ChannelType,
    pub external_conversation_id: T1,
    pub message_limit: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelMessage {
    pub id: i64,
    pub channel: crate::types::ChannelType,
    pub direction: crate::types::ChannelMessageDirection,
    pub external_conversation_id: String,
    pub message_text: String,
    pub status: crate::types::ChannelMessageStatus,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
pub struct ChannelMessageBorrowed<'a> {
    pub id: i64,
    pub channel: crate::types::ChannelType,
    pub direction: crate::types::ChannelMessageDirection,
    pub external_conversation_id: &'a str,
    pub message_text: &'a str,
    pub status: crate::types::ChannelMessageStatus,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl<'a> From<ChannelMessageBorrowed<'a>> for ChannelMessage {
    fn from(
        ChannelMessageBorrowed {
            id,
            channel,
            direction,
            external_conversation_id,
            message_text,
            status,
            created_at,
            updated_at,
        }: ChannelMessageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            channel,
            direction,
            external_conversation_id: external_conversation_id.into(),
            message_text: message_text.into(),
            status,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConversationMessage {
    pub id: i64,
    pub direction: crate::types::ChannelMessageDirection,
    pub message_text: String,
    pub status: crate::types::ChannelMessageStatus,
    pub created_at: chrono::NaiveDateTime,
}
pub struct ConversationMessageBorrowed<'a> {
    pub id: i64,
    pub direction: crate::types::ChannelMessageDirection,
    pub message_text: &'a str,
    pub status: crate::types::ChannelMessageStatus,
    pub created_at: chrono::NaiveDateTime,
}
impl<'a> From<ConversationMessageBorrowed<'a>> for ConversationMessage {
    fn from(
        ConversationMessageBorrowed {
            id,
            direction,
            message_text,
            status,
            created_at,
        }: ConversationMessageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            direction,
            message_text: message_text.into(),
            status,
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ChannelMessageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ChannelMessageBorrowed, tokio_postgres::Error>,
    mapper: fn(ChannelMessageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ChannelMessageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ChannelMessageBorrowed) -> R,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, R, N> {
        ChannelMessageQuery {
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
pub struct ConversationMessageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ConversationMessageBorrowed, tokio_postgres::Error>,
    mapper: fn(ConversationMessageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ConversationMessageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ConversationMessageBorrowed) -> R,
    ) -> ConversationMessageQuery<'c, 'a, 's, C, R, N> {
        ConversationMessageQuery {
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
pub struct InsertChannelMessageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_channel_message() -> InsertChannelMessageStmt {
    InsertChannelMessageStmt(
        "INSERT INTO channel_messages ( channel, direction, external_conversation_id, external_user_id, external_message_id, message_text, status, metadata_json ) VALUES ( $1::channel_type, $2::channel_message_direction, $3::TEXT, $4::TEXT, $5::TEXT, $6::TEXT, $7::channel_message_status, $8::JSONB ) RETURNING id, channel, direction, external_conversation_id, message_text, status, created_at, updated_at",
        None,
    )
}
impl InsertChannelMessageStmt {
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
        T5: crate::JsonSql,
    >(
        &'s self,
        client: &'c C,
        channel: &'a crate::types::ChannelType,
        direction: &'a crate::types::ChannelMessageDirection,
        external_conversation_id: &'a T1,
        external_user_id: &'a Option<T2>,
        external_message_id: &'a Option<T3>,
        message_text: &'a T4,
        status: &'a crate::types::ChannelMessageStatus,
        metadata_json: &'a T5,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 8> {
        ChannelMessageQuery {
            client,
            params: [
                channel,
                direction,
                external_conversation_id,
                external_user_id,
                external_message_id,
                message_text,
                status,
                metadata_json,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ChannelMessageBorrowed, tokio_postgres::Error> {
                Ok(ChannelMessageBorrowed {
                    id: row.try_get(0)?,
                    channel: row.try_get(1)?,
                    direction: row.try_get(2)?,
                    external_conversation_id: row.try_get(3)?,
                    message_text: row.try_get(4)?,
                    status: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| ChannelMessage::from(it),
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
    T5: crate::JsonSql,
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        InsertChannelMessageParams<T1, T2, T3, T4, T5>,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 8>,
        C,
    > for InsertChannelMessageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a InsertChannelMessageParams<T1, T2, T3, T4, T5>,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 8> {
        self.bind(
            client,
            &params.channel,
            &params.direction,
            &params.external_conversation_id,
            &params.external_user_id,
            &params.external_message_id,
            &params.message_text,
            &params.status,
            &params.metadata_json,
        )
    }
}
pub struct UpdateChannelMessageStatusStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_channel_message_status() -> UpdateChannelMessageStatusStmt {
    UpdateChannelMessageStatusStmt(
        "UPDATE channel_messages SET status = $1::channel_message_status, processed_at = CASE WHEN $1::channel_message_status = 'processed' THEN NOW() ELSE processed_at END, delivered_at = CASE WHEN $1::channel_message_status = 'sent' THEN NOW() ELSE delivered_at END, updated_at = NOW() WHERE id = $2::BIGINT RETURNING id, channel, direction, external_conversation_id, message_text, status, created_at, updated_at",
        None,
    )
}
impl UpdateChannelMessageStatusStmt {
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
        status: &'a crate::types::ChannelMessageStatus,
        id: &'a i64,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 2> {
        ChannelMessageQuery {
            client,
            params: [status, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ChannelMessageBorrowed, tokio_postgres::Error> {
                Ok(ChannelMessageBorrowed {
                    id: row.try_get(0)?,
                    channel: row.try_get(1)?,
                    direction: row.try_get(2)?,
                    external_conversation_id: row.try_get(3)?,
                    message_text: row.try_get(4)?,
                    status: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| ChannelMessage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateChannelMessageStatusParams,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 2>,
        C,
    > for UpdateChannelMessageStatusStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateChannelMessageStatusParams,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 2> {
        self.bind(client, &params.status, &params.id)
    }
}
pub struct ClaimNextChannelMessageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn claim_next_channel_message() -> ClaimNextChannelMessageStmt {
    ClaimNextChannelMessageStmt(
        "WITH next_message AS ( SELECT id FROM channel_messages WHERE channel = $1::channel_type AND direction = $2::channel_message_direction AND status = $3::channel_message_status ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED ) UPDATE channel_messages SET status = $4::channel_message_status, updated_at = NOW() WHERE id IN (SELECT id FROM next_message) RETURNING id, channel, direction, external_conversation_id, message_text, status, created_at, updated_at",
        None,
    )
}
impl ClaimNextChannelMessageStmt {
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
        channel: &'a crate::types::ChannelType,
        direction: &'a crate::types::ChannelMessageDirection,
        from_status: &'a crate::types::ChannelMessageStatus,
        to_status: &'a crate::types::ChannelMessageStatus,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 4> {
        ChannelMessageQuery {
            client,
            params: [channel, direction, from_status, to_status],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ChannelMessageBorrowed, tokio_postgres::Error> {
                Ok(ChannelMessageBorrowed {
                    id: row.try_get(0)?,
                    channel: row.try_get(1)?,
                    direction: row.try_get(2)?,
                    external_conversation_id: row.try_get(3)?,
                    message_text: row.try_get(4)?,
                    status: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    updated_at: row.try_get(7)?,
                })
            },
            mapper: |it| ChannelMessage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ClaimNextChannelMessageParams,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 4>,
        C,
    > for ClaimNextChannelMessageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ClaimNextChannelMessageParams,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 4> {
        self.bind(
            client,
            &params.channel,
            &params.direction,
            &params.from_status,
            &params.to_status,
        )
    }
}
pub struct ListConversationMessagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_conversation_messages() -> ListConversationMessagesStmt {
    ListConversationMessagesStmt(
        "SELECT id, direction, message_text, status, created_at FROM ( SELECT id, direction, message_text, status, created_at FROM channel_messages WHERE channel = $1::channel_type AND external_conversation_id = $2::TEXT ORDER BY created_at DESC LIMIT $3::BIGINT ) AS recent_messages ORDER BY created_at ASC",
        None,
    )
}
impl ListConversationMessagesStmt {
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
        channel: &'a crate::types::ChannelType,
        external_conversation_id: &'a T1,
        message_limit: &'a i64,
    ) -> ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 3> {
        ConversationMessageQuery {
            client,
            params: [channel, external_conversation_id, message_limit],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ConversationMessageBorrowed, tokio_postgres::Error> {
                Ok(ConversationMessageBorrowed {
                    id: row.try_get(0)?,
                    direction: row.try_get(1)?,
                    message_text: row.try_get(2)?,
                    status: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| ConversationMessage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ListConversationMessagesParams<T1>,
        ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 3>,
        C,
    > for ListConversationMessagesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ListConversationMessagesParams<T1>,
    ) -> ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 3> {
        self.bind(
            client,
            &params.channel,
            &params.external_conversation_id,
            &params.message_limit,
        )
    }
}
