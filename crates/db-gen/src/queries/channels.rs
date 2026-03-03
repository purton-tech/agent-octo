// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertChannelMessageParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::JsonSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
    T8: crate::StringSql,
> {
    pub channel: T1,
    pub external_conversation_id: T2,
    pub direction: T3,
    pub message_text: T4,
    pub metadata_json: T5,
    pub external_user_id: Option<T6>,
    pub external_message_id: Option<T7>,
    pub status: T8,
}
#[derive(Debug)]
pub struct UpdateChannelMessageStatusParams<T1: crate::StringSql> {
    pub status: T1,
    pub id: uuid::Uuid,
}
#[derive(Debug)]
pub struct ClaimNextChannelMessageParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub channel: T1,
    pub direction: T2,
    pub from_status: T3,
    pub to_status: T4,
}
#[derive(Debug)]
pub struct ListConversationMessagesParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub channel: T1,
    pub external_conversation_id: T2,
    pub message_limit: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelMessage {
    pub id: uuid::Uuid,
    pub channel: String,
    pub direction: String,
    pub external_conversation_id: String,
    pub message_text: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ChannelMessageBorrowed<'a> {
    pub id: uuid::Uuid,
    pub channel: &'a str,
    pub direction: &'a str,
    pub external_conversation_id: &'a str,
    pub message_text: &'a str,
    pub status: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
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
            channel: channel.into(),
            direction: direction.into(),
            external_conversation_id: external_conversation_id.into(),
            message_text: message_text.into(),
            status: status.into(),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConversationMessage {
    pub id: uuid::Uuid,
    pub direction: String,
    pub message_text: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ConversationMessageBorrowed<'a> {
    pub id: uuid::Uuid,
    pub direction: &'a str,
    pub message_text: &'a str,
    pub status: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
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
            direction: direction.into(),
            message_text: message_text.into(),
            status: status.into(),
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
        "WITH selected_channel AS ( SELECT c.org_id, c.created_by_user_id FROM public.channels c WHERE c.kind::TEXT = $1::TEXT ORDER BY c.created_at ASC LIMIT 1 ), existing_conversation AS ( SELECT c.id FROM public.conversations c WHERE c.title = $2::TEXT ORDER BY c.created_at ASC LIMIT 1 ), inserted_conversation AS ( INSERT INTO public.conversations ( org_id, created_by_user_id, title ) SELECT sc.org_id, sc.created_by_user_id, $2::TEXT FROM selected_channel sc WHERE NOT EXISTS (SELECT 1 FROM existing_conversation) RETURNING id ), resolved_conversation AS ( SELECT id FROM existing_conversation UNION ALL SELECT id FROM inserted_conversation ), inserted_message AS ( INSERT INTO public.messages ( conversation_id, role, content, metadata_json ) SELECT rc.id, CASE WHEN $3::TEXT = 'inbound' THEN 'user'::message_role ELSE 'assistant'::message_role END, $4::TEXT, COALESCE($5::JSONB, '{}'::JSONB) || jsonb_build_object( 'channel', $1::TEXT, 'direction', $3::TEXT, 'external_conversation_id', $2::TEXT, 'external_user_id', $6::TEXT, 'external_message_id', $7::TEXT, 'status', $8::TEXT, 'updated_at', NOW() ) FROM resolved_conversation rc RETURNING id, content, metadata_json, created_at ) SELECT m.id, m.metadata_json ->> 'channel' AS channel, m.metadata_json ->> 'direction' AS direction, m.metadata_json ->> 'external_conversation_id' AS external_conversation_id, m.content AS message_text, m.metadata_json ->> 'status' AS status, m.created_at, COALESCE((m.metadata_json ->> 'updated_at')::TIMESTAMPTZ, m.created_at) AS updated_at FROM inserted_message m",
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
        T6: crate::StringSql,
        T7: crate::StringSql,
        T8: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        channel: &'a T1,
        external_conversation_id: &'a T2,
        direction: &'a T3,
        message_text: &'a T4,
        metadata_json: &'a T5,
        external_user_id: &'a Option<T6>,
        external_message_id: &'a Option<T7>,
        status: &'a T8,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 8> {
        ChannelMessageQuery {
            client,
            params: [
                channel,
                external_conversation_id,
                direction,
                message_text,
                metadata_json,
                external_user_id,
                external_message_id,
                status,
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
        T6: crate::StringSql,
        T7: crate::StringSql,
        T8: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        InsertChannelMessageParams<T1, T2, T3, T4, T5, T6, T7, T8>,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 8>,
        C,
    > for InsertChannelMessageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a InsertChannelMessageParams<T1, T2, T3, T4, T5, T6, T7, T8>,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 8> {
        self.bind(
            client,
            &params.channel,
            &params.external_conversation_id,
            &params.direction,
            &params.message_text,
            &params.metadata_json,
            &params.external_user_id,
            &params.external_message_id,
            &params.status,
        )
    }
}
pub struct UpdateChannelMessageStatusStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_channel_message_status() -> UpdateChannelMessageStatusStmt {
    UpdateChannelMessageStatusStmt(
        "UPDATE public.messages SET metadata_json = CASE WHEN $1::TEXT = 'processed' THEN jsonb_set( jsonb_set( jsonb_set( metadata_json, '{status}', to_jsonb($1::TEXT), true ), '{processed_at}', to_jsonb(NOW()), true ), '{updated_at}', to_jsonb(NOW()), true ) WHEN $1::TEXT = 'sent' THEN jsonb_set( jsonb_set( jsonb_set( metadata_json, '{status}', to_jsonb($1::TEXT), true ), '{delivered_at}', to_jsonb(NOW()), true ), '{updated_at}', to_jsonb(NOW()), true ) ELSE jsonb_set( jsonb_set( metadata_json, '{status}', to_jsonb($1::TEXT), true ), '{updated_at}', to_jsonb(NOW()), true ) END WHERE id = $2::UUID RETURNING id, metadata_json ->> 'channel' AS channel, metadata_json ->> 'direction' AS direction, metadata_json ->> 'external_conversation_id' AS external_conversation_id, content AS message_text, metadata_json ->> 'status' AS status, created_at, COALESCE((metadata_json ->> 'updated_at')::TIMESTAMPTZ, created_at) AS updated_at",
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
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        status: &'a T1,
        id: &'a uuid::Uuid,
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
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateChannelMessageStatusParams<T1>,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 2>,
        C,
    > for UpdateChannelMessageStatusStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateChannelMessageStatusParams<T1>,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 2> {
        self.bind(client, &params.status, &params.id)
    }
}
pub struct ClaimNextChannelMessageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn claim_next_channel_message() -> ClaimNextChannelMessageStmt {
    ClaimNextChannelMessageStmt(
        "WITH next_message AS ( SELECT id FROM public.messages WHERE (metadata_json ->> 'channel') = $1::TEXT AND (metadata_json ->> 'direction') = $2::TEXT AND (metadata_json ->> 'status') = $3::TEXT ORDER BY created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED ) UPDATE public.messages SET metadata_json = jsonb_set( jsonb_set( metadata_json, '{status}', to_jsonb($4::TEXT), true ), '{updated_at}', to_jsonb(NOW()), true ) WHERE id IN (SELECT id FROM next_message) RETURNING id, metadata_json ->> 'channel' AS channel, metadata_json ->> 'direction' AS direction, metadata_json ->> 'external_conversation_id' AS external_conversation_id, content AS message_text, metadata_json ->> 'status' AS status, created_at, COALESCE((metadata_json ->> 'updated_at')::TIMESTAMPTZ, created_at) AS updated_at",
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
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        channel: &'a T1,
        direction: &'a T2,
        from_status: &'a T3,
        to_status: &'a T4,
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
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ClaimNextChannelMessageParams<T1, T2, T3, T4>,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 4>,
        C,
    > for ClaimNextChannelMessageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ClaimNextChannelMessageParams<T1, T2, T3, T4>,
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
        "SELECT recent_messages.id, recent_messages.direction, recent_messages.message_text, recent_messages.status, recent_messages.created_at FROM ( SELECT m.id, m.metadata_json ->> 'direction' AS direction, m.content AS message_text, m.metadata_json ->> 'status' AS status, m.created_at FROM public.messages m INNER JOIN public.conversations c ON c.id = m.conversation_id WHERE (m.metadata_json ->> 'channel') = $1::TEXT AND c.title = $2::TEXT ORDER BY m.created_at DESC LIMIT $3::BIGINT ) AS recent_messages ORDER BY recent_messages.created_at ASC",
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
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        channel: &'a T1,
        external_conversation_id: &'a T2,
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
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ListConversationMessagesParams<T1, T2>,
        ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 3>,
        C,
    > for ListConversationMessagesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ListConversationMessagesParams<T1, T2>,
    ) -> ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 3> {
        self.bind(
            client,
            &params.channel,
            &params.external_conversation_id,
            &params.message_limit,
        )
    }
}
