// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetOrCreateChannelConversationParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub channel: crate::types::ChannelType,
    pub external_user_id: Option<T1>,
    pub external_conversation_id: T2,
}
#[derive(Debug)]
pub struct InsertChannelMessageParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub external_message_id: Option<T1>,
    pub channel_conversation_id: uuid::Uuid,
    pub direction: crate::types::ChannelMessageDirection,
    pub message_text: T2,
    pub status: crate::types::ChannelMessageStatus,
}
#[derive(Clone, Copy, Debug)]
pub struct UpdateChannelMessageStatusParams {
    pub status: crate::types::ChannelMessageStatus,
    pub id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct ClaimNextChannelMessageParams {
    pub channel: crate::types::ChannelType,
    pub direction: crate::types::ChannelMessageDirection,
    pub from_status: crate::types::ChannelMessageStatus,
    pub to_status: crate::types::ChannelMessageStatus,
}
#[derive(Clone, Copy, Debug)]
pub struct ListConversationMessagesParams {
    pub conversation_id: uuid::Uuid,
    pub message_limit: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelConfig {
    pub id: uuid::Uuid,
    pub bot_token: String,
}
pub struct ChannelConfigBorrowed<'a> {
    pub id: uuid::Uuid,
    pub bot_token: &'a str,
}
impl<'a> From<ChannelConfigBorrowed<'a>> for ChannelConfig {
    fn from(ChannelConfigBorrowed { id, bot_token }: ChannelConfigBorrowed<'a>) -> Self {
        Self {
            id,
            bot_token: bot_token.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelConversation {
    pub id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub external_conversation_id: String,
    pub agent_id: uuid::Uuid,
}
pub struct ChannelConversationBorrowed<'a> {
    pub id: uuid::Uuid,
    pub channel_id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub external_conversation_id: &'a str,
    pub agent_id: uuid::Uuid,
}
impl<'a> From<ChannelConversationBorrowed<'a>> for ChannelConversation {
    fn from(
        ChannelConversationBorrowed {
            id,
            channel_id,
            conversation_id,
            external_conversation_id,
            agent_id,
        }: ChannelConversationBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            channel_id,
            conversation_id,
            external_conversation_id: external_conversation_id.into(),
            agent_id,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelMessage {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub channel_conversation_id: uuid::Uuid,
    pub direction: crate::types::ChannelMessageDirection,
    pub external_conversation_id: String,
    pub message_text: String,
    pub status: crate::types::ChannelMessageStatus,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ChannelMessageBorrowed<'a> {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub channel_conversation_id: uuid::Uuid,
    pub direction: crate::types::ChannelMessageDirection,
    pub external_conversation_id: &'a str,
    pub message_text: &'a str,
    pub status: crate::types::ChannelMessageStatus,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<ChannelMessageBorrowed<'a>> for ChannelMessage {
    fn from(
        ChannelMessageBorrowed {
            id,
            conversation_id,
            channel_conversation_id,
            direction,
            external_conversation_id,
            message_text,
            status,
            created_at,
        }: ChannelMessageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            conversation_id,
            channel_conversation_id,
            direction,
            external_conversation_id: external_conversation_id.into(),
            message_text: message_text.into(),
            status,
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConversationMessage {
    pub id: uuid::Uuid,
    pub direction: crate::types::ChannelMessageDirection,
    pub message_text: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ConversationMessageBorrowed<'a> {
    pub id: uuid::Uuid,
    pub direction: crate::types::ChannelMessageDirection,
    pub message_text: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<ConversationMessageBorrowed<'a>> for ConversationMessage {
    fn from(
        ConversationMessageBorrowed {
            id,
            direction,
            message_text,
            created_at,
        }: ConversationMessageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            direction,
            message_text: message_text.into(),
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ChannelConfigQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ChannelConfigBorrowed, tokio_postgres::Error>,
    mapper: fn(ChannelConfigBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ChannelConfigQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ChannelConfigBorrowed) -> R,
    ) -> ChannelConfigQuery<'c, 'a, 's, C, R, N> {
        ChannelConfigQuery {
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
pub struct ChannelConversationQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ChannelConversationBorrowed, tokio_postgres::Error>,
    mapper: fn(ChannelConversationBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ChannelConversationQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ChannelConversationBorrowed) -> R,
    ) -> ChannelConversationQuery<'c, 'a, 's, C, R, N> {
        ChannelConversationQuery {
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
pub struct GetChannelConfigStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_channel_config() -> GetChannelConfigStmt {
    GetChannelConfigStmt(
        "SELECT c.id, c.bot_token FROM public.channels c WHERE c.kind = $1::channel_type ORDER BY c.created_at ASC LIMIT 1",
        None,
    )
}
impl GetChannelConfigStmt {
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
    ) -> ChannelConfigQuery<'c, 'a, 's, C, ChannelConfig, 1> {
        ChannelConfigQuery {
            client,
            params: [channel],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ChannelConfigBorrowed, tokio_postgres::Error> {
                    Ok(ChannelConfigBorrowed {
                        id: row.try_get(0)?,
                        bot_token: row.try_get(1)?,
                    })
                },
            mapper: |it| ChannelConfig::from(it),
        }
    }
}
pub struct GetOrCreateChannelConversationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_or_create_channel_conversation() -> GetOrCreateChannelConversationStmt {
    GetOrCreateChannelConversationStmt(
        "WITH selected_channel AS ( SELECT c.id, c.org_id, c.created_by_user_id, c.default_agent_id FROM public.channels c WHERE c.kind = $1::channel_type ORDER BY c.created_at ASC LIMIT 1 ), updated_binding AS ( UPDATE public.channel_conversations cc SET external_user_id = COALESCE($2::TEXT, cc.external_user_id), updated_at = NOW() FROM selected_channel sc WHERE cc.channel_id = sc.id AND cc.external_conversation_id = $3::TEXT RETURNING cc.id, cc.channel_id, cc.conversation_id, cc.external_conversation_id ), inserted_conversation AS ( INSERT INTO public.conversations ( org_id, created_by_user_id, agent_id, title ) SELECT sc.org_id, sc.created_by_user_id, sc.default_agent_id, NULL FROM selected_channel sc WHERE sc.default_agent_id IS NOT NULL AND NOT EXISTS (SELECT 1 FROM updated_binding) RETURNING id ), inserted_binding AS ( INSERT INTO public.channel_conversations ( channel_id, conversation_id, external_conversation_id, external_user_id ) SELECT sc.id, ic.id, $3::TEXT, $2::TEXT FROM selected_channel sc INNER JOIN inserted_conversation ic ON TRUE RETURNING id, channel_id, conversation_id, external_conversation_id ), resolved_binding AS ( SELECT * FROM updated_binding UNION ALL SELECT * FROM inserted_binding ) SELECT rb.id, rb.channel_id, rb.conversation_id, rb.external_conversation_id, c.agent_id FROM resolved_binding rb INNER JOIN public.conversations c ON c.id = rb.conversation_id LIMIT 1",
        None,
    )
}
impl GetOrCreateChannelConversationStmt {
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
        channel: &'a crate::types::ChannelType,
        external_user_id: &'a Option<T1>,
        external_conversation_id: &'a T2,
    ) -> ChannelConversationQuery<'c, 'a, 's, C, ChannelConversation, 3> {
        ChannelConversationQuery {
            client,
            params: [channel, external_user_id, external_conversation_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ChannelConversationBorrowed, tokio_postgres::Error> {
                Ok(ChannelConversationBorrowed {
                    id: row.try_get(0)?,
                    channel_id: row.try_get(1)?,
                    conversation_id: row.try_get(2)?,
                    external_conversation_id: row.try_get(3)?,
                    agent_id: row.try_get(4)?,
                })
            },
            mapper: |it| ChannelConversation::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetOrCreateChannelConversationParams<T1, T2>,
        ChannelConversationQuery<'c, 'a, 's, C, ChannelConversation, 3>,
        C,
    > for GetOrCreateChannelConversationStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetOrCreateChannelConversationParams<T1, T2>,
    ) -> ChannelConversationQuery<'c, 'a, 's, C, ChannelConversation, 3> {
        self.bind(
            client,
            &params.channel,
            &params.external_user_id,
            &params.external_conversation_id,
        )
    }
}
pub struct InsertChannelMessageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_channel_message() -> InsertChannelMessageStmt {
    InsertChannelMessageStmt(
        "WITH updated_binding AS ( UPDATE public.channel_conversations cc SET last_external_message_id = COALESCE( $1::TEXT, cc.last_external_message_id ), updated_at = NOW() WHERE cc.id = $2::UUID RETURNING cc.id, cc.conversation_id, cc.external_conversation_id ), inserted_message AS ( INSERT INTO public.messages ( conversation_id, role, content, channel_conversation_id, channel_message_direction, channel_message_status, external_message_id ) SELECT ub.conversation_id, CASE WHEN $3::channel_message_direction = 'inbound' THEN 'user'::message_role ELSE 'assistant'::message_role END, $4::TEXT, ub.id, $3::channel_message_direction, $5::channel_message_status, $1::TEXT FROM updated_binding ub RETURNING id, conversation_id, channel_conversation_id, channel_message_direction, content, channel_message_status, created_at ) SELECT im.id, im.conversation_id, im.channel_conversation_id, im.channel_message_direction AS direction, ub.external_conversation_id, im.content AS message_text, im.channel_message_status AS status, im.created_at FROM inserted_message im INNER JOIN updated_binding ub ON ub.id = im.channel_conversation_id",
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
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        external_message_id: &'a Option<T1>,
        channel_conversation_id: &'a uuid::Uuid,
        direction: &'a crate::types::ChannelMessageDirection,
        message_text: &'a T2,
        status: &'a crate::types::ChannelMessageStatus,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 5> {
        ChannelMessageQuery {
            client,
            params: [
                external_message_id,
                channel_conversation_id,
                direction,
                message_text,
                status,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ChannelMessageBorrowed, tokio_postgres::Error> {
                Ok(ChannelMessageBorrowed {
                    id: row.try_get(0)?,
                    conversation_id: row.try_get(1)?,
                    channel_conversation_id: row.try_get(2)?,
                    direction: row.try_get(3)?,
                    external_conversation_id: row.try_get(4)?,
                    message_text: row.try_get(5)?,
                    status: row.try_get(6)?,
                    created_at: row.try_get(7)?,
                })
            },
            mapper: |it| ChannelMessage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        InsertChannelMessageParams<T1, T2>,
        ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 5>,
        C,
    > for InsertChannelMessageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a InsertChannelMessageParams<T1, T2>,
    ) -> ChannelMessageQuery<'c, 'a, 's, C, ChannelMessage, 5> {
        self.bind(
            client,
            &params.external_message_id,
            &params.channel_conversation_id,
            &params.direction,
            &params.message_text,
            &params.status,
        )
    }
}
pub struct UpdateChannelMessageStatusStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_channel_message_status() -> UpdateChannelMessageStatusStmt {
    UpdateChannelMessageStatusStmt(
        "UPDATE public.messages m SET channel_message_status = $1::channel_message_status FROM public.channel_conversations cc WHERE m.id = $2::UUID AND cc.id = m.channel_conversation_id RETURNING m.id, m.conversation_id, m.channel_conversation_id, m.channel_message_direction AS direction, cc.external_conversation_id, m.content AS message_text, m.channel_message_status AS status, m.created_at",
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
                    conversation_id: row.try_get(1)?,
                    channel_conversation_id: row.try_get(2)?,
                    direction: row.try_get(3)?,
                    external_conversation_id: row.try_get(4)?,
                    message_text: row.try_get(5)?,
                    status: row.try_get(6)?,
                    created_at: row.try_get(7)?,
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
        "WITH next_message AS ( SELECT m.id FROM public.messages m INNER JOIN public.channel_conversations cc ON cc.id = m.channel_conversation_id INNER JOIN public.channels c ON c.id = cc.channel_id WHERE c.kind = $1::channel_type AND m.channel_message_direction = $2::channel_message_direction AND m.channel_message_status = $3::channel_message_status ORDER BY m.created_at ASC LIMIT 1 FOR UPDATE OF m SKIP LOCKED ) UPDATE public.messages m SET channel_message_status = $4::channel_message_status FROM public.channel_conversations cc WHERE m.id IN (SELECT id FROM next_message) AND cc.id = m.channel_conversation_id RETURNING m.id, m.conversation_id, m.channel_conversation_id, m.channel_message_direction AS direction, cc.external_conversation_id, m.content AS message_text, m.channel_message_status AS status, m.created_at",
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
                    conversation_id: row.try_get(1)?,
                    channel_conversation_id: row.try_get(2)?,
                    direction: row.try_get(3)?,
                    external_conversation_id: row.try_get(4)?,
                    message_text: row.try_get(5)?,
                    status: row.try_get(6)?,
                    created_at: row.try_get(7)?,
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
        "SELECT recent_messages.id, recent_messages.direction, recent_messages.message_text, recent_messages.created_at FROM ( SELECT m.id, m.channel_message_direction AS direction, m.content AS message_text, m.created_at FROM public.messages m WHERE m.conversation_id = $1::UUID AND m.channel_message_direction IS NOT NULL ORDER BY m.created_at DESC LIMIT $2::BIGINT ) AS recent_messages ORDER BY recent_messages.created_at ASC",
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
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        conversation_id: &'a uuid::Uuid,
        message_limit: &'a i64,
    ) -> ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 2> {
        ConversationMessageQuery {
            client,
            params: [conversation_id, message_limit],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ConversationMessageBorrowed, tokio_postgres::Error> {
                Ok(ConversationMessageBorrowed {
                    id: row.try_get(0)?,
                    direction: row.try_get(1)?,
                    message_text: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                })
            },
            mapper: |it| ConversationMessage::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ListConversationMessagesParams,
        ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 2>,
        C,
    > for ListConversationMessagesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ListConversationMessagesParams,
    ) -> ConversationMessageQuery<'c, 'a, 's, C, ConversationMessage, 2> {
        self.bind(client, &params.conversation_id, &params.message_limit)
    }
}
