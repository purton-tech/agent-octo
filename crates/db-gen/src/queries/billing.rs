// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct RecordLlmUsageForConversationParams {
    pub conversation_id: uuid::Uuid,
    pub input_tokens: i64,
    pub output_tokens: i64,
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
