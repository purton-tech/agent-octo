// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct RecordLlmUsageForConversationParams {
    pub conversation_id: uuid::Uuid,
    pub input_tokens: i64,
    pub output_tokens: i64,
}
#[derive(Debug)]
pub struct UpsertStripeCustomerForOrgParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub org_id: T1,
    pub stripe_customer_id: T2,
}
#[derive(Debug)]
pub struct CreateTopUpTransactionParams<T1: crate::StringSql> {
    pub org_id: T1,
    pub user_id: uuid::Uuid,
    pub amount_microcents: i64,
}
#[derive(Debug)]
pub struct AttachTopUpCheckoutSessionParams<T1: crate::StringSql> {
    pub stripe_checkout_session_id: T1,
    pub transaction_id: uuid::Uuid,
}
#[derive(Debug)]
pub struct GetTopUpTransactionForOrgParams<T1: crate::StringSql> {
    pub transaction_id: uuid::Uuid,
    pub org_id: T1,
}
#[derive(Debug)]
pub struct CompleteTopUpCheckoutSessionParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
> {
    pub stripe_payment_intent: Option<T1>,
    pub stripe_event_id: T2,
    pub stripe_checkout_session_id: T3,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct UsageCharge {
    pub id: uuid::Uuid,
    pub org_id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub cost_microcents: i64,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StripeCustomerRecord {
    pub stripe_customer_id: String,
}
pub struct StripeCustomerRecordBorrowed<'a> {
    pub stripe_customer_id: &'a str,
}
impl<'a> From<StripeCustomerRecordBorrowed<'a>> for StripeCustomerRecord {
    fn from(
        StripeCustomerRecordBorrowed { stripe_customer_id }: StripeCustomerRecordBorrowed<'a>,
    ) -> Self {
        Self {
            stripe_customer_id: stripe_customer_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct TopUpTransactionId {
    pub id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TopUpTransaction {
    pub amount_microcents: i64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub completed_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct TopUpTransactionBorrowed<'a> {
    pub amount_microcents: i64,
    pub status: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub completed_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<TopUpTransactionBorrowed<'a>> for TopUpTransaction {
    fn from(
        TopUpTransactionBorrowed {
            amount_microcents,
            status,
            created_at,
            completed_at,
        }: TopUpTransactionBorrowed<'a>,
    ) -> Self {
        Self {
            amount_microcents,
            status: status.into(),
            created_at,
            completed_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TopUpTransactionStatus {
    pub status: String,
    pub stripe_checkout_session_id: String,
}
pub struct TopUpTransactionStatusBorrowed<'a> {
    pub status: &'a str,
    pub stripe_checkout_session_id: &'a str,
}
impl<'a> From<TopUpTransactionStatusBorrowed<'a>> for TopUpTransactionStatus {
    fn from(
        TopUpTransactionStatusBorrowed {
            status,
            stripe_checkout_session_id,
        }: TopUpTransactionStatusBorrowed<'a>,
    ) -> Self {
        Self {
            status: status.into(),
            stripe_checkout_session_id: stripe_checkout_session_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TopUpCompletion {
    pub applied: bool,
    pub org_public_id: String,
    pub amount_microcents: i64,
}
pub struct TopUpCompletionBorrowed<'a> {
    pub applied: bool,
    pub org_public_id: &'a str,
    pub amount_microcents: i64,
}
impl<'a> From<TopUpCompletionBorrowed<'a>> for TopUpCompletion {
    fn from(
        TopUpCompletionBorrowed {
            applied,
            org_public_id,
            amount_microcents,
        }: TopUpCompletionBorrowed<'a>,
    ) -> Self {
        Self {
            applied,
            org_public_id: org_public_id.into(),
            amount_microcents,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UsageChargeQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UsageCharge, tokio_postgres::Error>,
    mapper: fn(UsageCharge) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UsageChargeQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UsageCharge) -> R) -> UsageChargeQuery<'c, 'a, 's, C, R, N> {
        UsageChargeQuery {
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
pub struct StripeCustomerRecordQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<StripeCustomerRecordBorrowed, tokio_postgres::Error>,
    mapper: fn(StripeCustomerRecordBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StripeCustomerRecordQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(StripeCustomerRecordBorrowed) -> R,
    ) -> StripeCustomerRecordQuery<'c, 'a, 's, C, R, N> {
        StripeCustomerRecordQuery {
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
pub struct TopUpTransactionIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TopUpTransactionId, tokio_postgres::Error>,
    mapper: fn(TopUpTransactionId) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TopUpTransactionIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(TopUpTransactionId) -> R,
    ) -> TopUpTransactionIdQuery<'c, 'a, 's, C, R, N> {
        TopUpTransactionIdQuery {
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
pub struct TopUpTransactionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TopUpTransactionBorrowed, tokio_postgres::Error>,
    mapper: fn(TopUpTransactionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TopUpTransactionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(TopUpTransactionBorrowed) -> R,
    ) -> TopUpTransactionQuery<'c, 'a, 's, C, R, N> {
        TopUpTransactionQuery {
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
pub struct TopUpTransactionStatusQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<TopUpTransactionStatusBorrowed, tokio_postgres::Error>,
    mapper: fn(TopUpTransactionStatusBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TopUpTransactionStatusQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(TopUpTransactionStatusBorrowed) -> R,
    ) -> TopUpTransactionStatusQuery<'c, 'a, 's, C, R, N> {
        TopUpTransactionStatusQuery {
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
pub struct TopUpCompletionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TopUpCompletionBorrowed, tokio_postgres::Error>,
    mapper: fn(TopUpCompletionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TopUpCompletionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(TopUpCompletionBorrowed) -> R,
    ) -> TopUpCompletionQuery<'c, 'a, 's, C, R, N> {
        TopUpCompletionQuery {
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
pub struct RecordLlmUsageForConversationStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn record_llm_usage_for_conversation() -> RecordLlmUsageForConversationStmt {
    RecordLlmUsageForConversationStmt(
        "WITH billing_context AS ( SELECT c.org_id, p.price_per_million_input_microcents AS input_price_microcents, p.price_per_million_output_microcents AS output_price_microcents FROM public.conversations c INNER JOIN public.agent_llm al ON al.agent_id = c.agent_id INNER JOIN public.providers p ON p.id = al.provider_id WHERE c.id = $1::UUID ), computed_charge AS ( SELECT org_id, ( ($2::BIGINT * input_price_microcents) / 1000000 ) + ( ($3::BIGINT * output_price_microcents) / 1000000 ) AS cost_microcents FROM billing_context ), inserted_usage AS ( INSERT INTO public.llm_usage_events ( org_id, conversation_id, input_tokens, output_tokens, cost_microcents ) SELECT cc.org_id, $1::UUID, $2::INT, $3::INT, cc.cost_microcents FROM computed_charge cc RETURNING id, org_id, conversation_id, input_tokens, output_tokens, cost_microcents, created_at ), updated_org AS ( UPDATE org.orgs o SET balance_microcents = o.balance_microcents - iu.cost_microcents FROM inserted_usage iu WHERE o.id = iu.org_id RETURNING o.balance_microcents ) SELECT iu.id, iu.org_id, iu.conversation_id, iu.input_tokens, iu.output_tokens, iu.cost_microcents, iu.created_at FROM inserted_usage iu INNER JOIN updated_org uo ON TRUE",
        None,
    )
}
impl RecordLlmUsageForConversationStmt {
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
        input_tokens: &'a i64,
        output_tokens: &'a i64,
    ) -> UsageChargeQuery<'c, 'a, 's, C, UsageCharge, 3> {
        UsageChargeQuery {
            client,
            params: [conversation_id, input_tokens, output_tokens],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<UsageCharge, tokio_postgres::Error> {
                Ok(UsageCharge {
                    id: row.try_get(0)?,
                    org_id: row.try_get(1)?,
                    conversation_id: row.try_get(2)?,
                    input_tokens: row.try_get(3)?,
                    output_tokens: row.try_get(4)?,
                    cost_microcents: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                })
            },
            mapper: |it| UsageCharge::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        RecordLlmUsageForConversationParams,
        UsageChargeQuery<'c, 'a, 's, C, UsageCharge, 3>,
        C,
    > for RecordLlmUsageForConversationStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a RecordLlmUsageForConversationParams,
    ) -> UsageChargeQuery<'c, 'a, 's, C, UsageCharge, 3> {
        self.bind(
            client,
            &params.conversation_id,
            &params.input_tokens,
            &params.output_tokens,
        )
    }
}
pub struct GetStripeCustomerForOrgStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_stripe_customer_for_org() -> GetStripeCustomerForOrgStmt {
    GetStripeCustomerForOrgStmt(
        "SELECT stripe_customer_id FROM billing.stripe_customers WHERE org_id = public.b64url_to_uuid($1::TEXT)",
        None,
    )
}
impl GetStripeCustomerForOrgStmt {
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
    ) -> StripeCustomerRecordQuery<'c, 'a, 's, C, StripeCustomerRecord, 1> {
        StripeCustomerRecordQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<StripeCustomerRecordBorrowed, tokio_postgres::Error> {
                Ok(StripeCustomerRecordBorrowed {
                    stripe_customer_id: row.try_get(0)?,
                })
            },
            mapper: |it| StripeCustomerRecord::from(it),
        }
    }
}
pub struct UpsertStripeCustomerForOrgStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert_stripe_customer_for_org() -> UpsertStripeCustomerForOrgStmt {
    UpsertStripeCustomerForOrgStmt(
        "INSERT INTO billing.stripe_customers ( org_id, stripe_customer_id ) VALUES ( public.b64url_to_uuid($1::TEXT), $2::TEXT ) ON CONFLICT (org_id) DO UPDATE SET stripe_customer_id = EXCLUDED.stripe_customer_id, updated_at = NOW() RETURNING stripe_customer_id",
        None,
    )
}
impl UpsertStripeCustomerForOrgStmt {
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
        stripe_customer_id: &'a T2,
    ) -> StripeCustomerRecordQuery<'c, 'a, 's, C, StripeCustomerRecord, 2> {
        StripeCustomerRecordQuery {
            client,
            params: [org_id, stripe_customer_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<StripeCustomerRecordBorrowed, tokio_postgres::Error> {
                Ok(StripeCustomerRecordBorrowed {
                    stripe_customer_id: row.try_get(0)?,
                })
            },
            mapper: |it| StripeCustomerRecord::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpsertStripeCustomerForOrgParams<T1, T2>,
        StripeCustomerRecordQuery<'c, 'a, 's, C, StripeCustomerRecord, 2>,
        C,
    > for UpsertStripeCustomerForOrgStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpsertStripeCustomerForOrgParams<T1, T2>,
    ) -> StripeCustomerRecordQuery<'c, 'a, 's, C, StripeCustomerRecord, 2> {
        self.bind(client, &params.org_id, &params.stripe_customer_id)
    }
}
pub struct CreateTopUpTransactionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_top_up_transaction() -> CreateTopUpTransactionStmt {
    CreateTopUpTransactionStmt(
        "INSERT INTO billing.top_up_transactions ( org_id, user_id, amount_microcents, status ) VALUES ( public.b64url_to_uuid($1::TEXT), $2::UUID, $3::BIGINT, 'pending' ) RETURNING id",
        None,
    )
}
impl CreateTopUpTransactionStmt {
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
        user_id: &'a uuid::Uuid,
        amount_microcents: &'a i64,
    ) -> TopUpTransactionIdQuery<'c, 'a, 's, C, TopUpTransactionId, 3> {
        TopUpTransactionIdQuery {
            client,
            params: [org_id, user_id, amount_microcents],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<TopUpTransactionId, tokio_postgres::Error> {
                    Ok(TopUpTransactionId {
                        id: row.try_get(0)?,
                    })
                },
            mapper: |it| TopUpTransactionId::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateTopUpTransactionParams<T1>,
        TopUpTransactionIdQuery<'c, 'a, 's, C, TopUpTransactionId, 3>,
        C,
    > for CreateTopUpTransactionStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateTopUpTransactionParams<T1>,
    ) -> TopUpTransactionIdQuery<'c, 'a, 's, C, TopUpTransactionId, 3> {
        self.bind(
            client,
            &params.org_id,
            &params.user_id,
            &params.amount_microcents,
        )
    }
}
pub struct AttachTopUpCheckoutSessionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn attach_top_up_checkout_session() -> AttachTopUpCheckoutSessionStmt {
    AttachTopUpCheckoutSessionStmt(
        "UPDATE billing.top_up_transactions SET stripe_checkout_session_id = $1::TEXT WHERE id = $2::UUID",
        None,
    )
}
impl AttachTopUpCheckoutSessionStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        stripe_checkout_session_id: &'a T1,
        transaction_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[stripe_checkout_session_id, transaction_id])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        AttachTopUpCheckoutSessionParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for AttachTopUpCheckoutSessionStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a AttachTopUpCheckoutSessionParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.stripe_checkout_session_id,
            &params.transaction_id,
        ))
    }
}
pub struct MarkTopUpTransactionFailedStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn mark_top_up_transaction_failed() -> MarkTopUpTransactionFailedStmt {
    MarkTopUpTransactionFailedStmt(
        "UPDATE billing.top_up_transactions SET status = 'failed' WHERE id = $1::UUID AND status = 'pending'",
        None,
    )
}
impl MarkTopUpTransactionFailedStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        transaction_id: &'a uuid::Uuid,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[transaction_id]).await
    }
}
pub struct ListTopUpTransactionsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_top_up_transactions() -> ListTopUpTransactionsStmt {
    ListTopUpTransactionsStmt(
        "SELECT amount_microcents, status, created_at, COALESCE(completed_at, created_at) AS completed_at FROM billing.top_up_transactions WHERE org_id = public.b64url_to_uuid($1::TEXT) ORDER BY created_at DESC",
        None,
    )
}
impl ListTopUpTransactionsStmt {
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
    ) -> TopUpTransactionQuery<'c, 'a, 's, C, TopUpTransaction, 1> {
        TopUpTransactionQuery {
            client,
            params: [org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<TopUpTransactionBorrowed, tokio_postgres::Error> {
                Ok(TopUpTransactionBorrowed {
                    amount_microcents: row.try_get(0)?,
                    status: row.try_get(1)?,
                    created_at: row.try_get(2)?,
                    completed_at: row.try_get(3)?,
                })
            },
            mapper: |it| TopUpTransaction::from(it),
        }
    }
}
pub struct GetTopUpTransactionForOrgStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_top_up_transaction_for_org() -> GetTopUpTransactionForOrgStmt {
    GetTopUpTransactionForOrgStmt(
        "SELECT status, COALESCE(stripe_checkout_session_id, '') AS stripe_checkout_session_id FROM billing.top_up_transactions WHERE id = $1::UUID AND org_id = public.b64url_to_uuid($2::TEXT)",
        None,
    )
}
impl GetTopUpTransactionForOrgStmt {
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
        transaction_id: &'a uuid::Uuid,
        org_id: &'a T1,
    ) -> TopUpTransactionStatusQuery<'c, 'a, 's, C, TopUpTransactionStatus, 2> {
        TopUpTransactionStatusQuery {
            client,
            params: [transaction_id, org_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<TopUpTransactionStatusBorrowed, tokio_postgres::Error> {
                Ok(TopUpTransactionStatusBorrowed {
                    status: row.try_get(0)?,
                    stripe_checkout_session_id: row.try_get(1)?,
                })
            },
            mapper: |it| TopUpTransactionStatus::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetTopUpTransactionForOrgParams<T1>,
        TopUpTransactionStatusQuery<'c, 'a, 's, C, TopUpTransactionStatus, 2>,
        C,
    > for GetTopUpTransactionForOrgStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetTopUpTransactionForOrgParams<T1>,
    ) -> TopUpTransactionStatusQuery<'c, 'a, 's, C, TopUpTransactionStatus, 2> {
        self.bind(client, &params.transaction_id, &params.org_id)
    }
}
pub struct CompleteTopUpCheckoutSessionStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn complete_top_up_checkout_session() -> CompleteTopUpCheckoutSessionStmt {
    CompleteTopUpCheckoutSessionStmt(
        "WITH updated_transaction AS ( UPDATE billing.top_up_transactions SET status = 'succeeded', stripe_payment_intent_id = COALESCE( $1::TEXT, stripe_payment_intent_id ), stripe_event_id = $2::TEXT, completed_at = NOW() WHERE stripe_checkout_session_id = $3::TEXT AND status = 'pending' RETURNING org_id, amount_microcents ), updated_org AS ( UPDATE org.orgs o SET balance_microcents = o.balance_microcents + ut.amount_microcents FROM updated_transaction ut WHERE o.id = ut.org_id RETURNING ut.org_id, ut.amount_microcents ) SELECT EXISTS(SELECT 1 FROM updated_org) AS applied, COALESCE( ( SELECT public.uuid_to_b64url(org_id) FROM updated_org LIMIT 1 ), '' ) AS org_public_id, COALESCE( ( SELECT amount_microcents FROM updated_org LIMIT 1 ), 0 ) AS amount_microcents",
        None,
    )
}
impl CompleteTopUpCheckoutSessionStmt {
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
    >(
        &'s self,
        client: &'c C,
        stripe_payment_intent: &'a Option<T1>,
        stripe_event_id: &'a T2,
        stripe_checkout_session_id: &'a T3,
    ) -> TopUpCompletionQuery<'c, 'a, 's, C, TopUpCompletion, 3> {
        TopUpCompletionQuery {
            client,
            params: [stripe_payment_intent, stripe_event_id, stripe_checkout_session_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<TopUpCompletionBorrowed, tokio_postgres::Error> {
                Ok(TopUpCompletionBorrowed {
                    applied: row.try_get(0)?,
                    org_public_id: row.try_get(1)?,
                    amount_microcents: row.try_get(2)?,
                })
            },
            mapper: |it| TopUpCompletion::from(it),
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
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CompleteTopUpCheckoutSessionParams<T1, T2, T3>,
        TopUpCompletionQuery<'c, 'a, 's, C, TopUpCompletion, 3>,
        C,
    > for CompleteTopUpCheckoutSessionStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CompleteTopUpCheckoutSessionParams<T1, T2, T3>,
    ) -> TopUpCompletionQuery<'c, 'a, 's, C, TopUpCompletion, 3> {
        self.bind(
            client,
            &params.stripe_payment_intent,
            &params.stripe_event_id,
            &params.stripe_checkout_session_id,
        )
    }
}
