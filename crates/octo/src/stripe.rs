use crate::{CustomError, config::Config};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::Deserialize;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Deserialize)]
pub struct CreateCheckoutSessionResponse {
    pub id: String,
    pub url: String,
}

pub struct CheckoutSessionRequest<'a> {
    pub customer_id: &'a str,
    pub org_name: &'a str,
    pub org_id: &'a str,
    pub transaction_id: &'a str,
    pub amount_cents: i64,
    pub success_url: &'a str,
    pub cancel_url: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomerResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct StripeWebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: StripeWebhookData,
}

#[derive(Debug, Deserialize)]
pub struct StripeWebhookData {
    pub object: StripeCheckoutSessionObject,
}

#[derive(Debug, Deserialize)]
pub struct StripeCheckoutSessionObject {
    pub id: String,
    pub payment_status: Option<String>,
    pub payment_intent: Option<ExpandableField>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ExpandableField {
    Id(String),
    Object { id: String },
}

impl ExpandableField {
    pub fn id(&self) -> &str {
        match self {
            ExpandableField::Id(id) => id,
            ExpandableField::Object { id } => id,
        }
    }
}

pub async fn create_customer(
    config: &Config,
    email: &str,
    org_name: &str,
    org_id: &str,
) -> Result<CreateCustomerResponse, CustomError> {
    let secret_key = stripe_secret_key(config)?;
    let response = Client::new()
        .post("https://api.stripe.com/v1/customers")
        .basic_auth(secret_key, Some(""))
        .form(&[
            ("email", email.to_string()),
            ("name", org_name.to_string()),
            ("metadata[org_id]", org_id.to_string()),
        ])
        .send()
        .await
        .map_err(|err| {
            CustomError::FaultySetup(format!("failed to create Stripe customer: {err}"))
        })?;

    parse_stripe_response(response, "create Stripe customer").await
}

pub async fn create_checkout_session(
    config: &Config,
    request: CheckoutSessionRequest<'_>,
) -> Result<CreateCheckoutSessionResponse, CustomError> {
    let secret_key = stripe_secret_key(config)?;
    let amount_label = format!("${}.00", request.amount_cents / 100);
    let response = Client::new()
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(secret_key, Some(""))
        .form(&[
            ("mode", "payment".to_string()),
            ("success_url", request.success_url.to_string()),
            ("cancel_url", request.cancel_url.to_string()),
            ("customer", request.customer_id.to_string()),
            ("client_reference_id", request.org_id.to_string()),
            ("metadata[org_id]", request.org_id.to_string()),
            (
                "metadata[top_up_transaction_id]",
                request.transaction_id.to_string(),
            ),
            ("line_items[0][price_data][currency]", "usd".to_string()),
            (
                "line_items[0][price_data][product_data][name]",
                "Agent Octo Balance Top Up".to_string(),
            ),
            (
                "line_items[0][price_data][product_data][description]",
                format!("{amount_label} prepaid balance for {}", request.org_name),
            ),
            (
                "line_items[0][price_data][unit_amount]",
                request.amount_cents.to_string(),
            ),
            ("line_items[0][quantity]", "1".to_string()),
        ])
        .send()
        .await
        .map_err(|err| {
            CustomError::FaultySetup(format!("failed to create Stripe Checkout session: {err}"))
        })?;

    parse_stripe_response(response, "create Stripe Checkout session").await
}

pub fn verify_webhook(
    config: &Config,
    signature_header: &str,
    payload: &[u8],
) -> Result<StripeWebhookEvent, CustomError> {
    let secret = config.stripe_webhook_secret.as_deref().ok_or_else(|| {
        CustomError::FaultySetup("STRIPE_WEBHOOK_SECRET is not configured".to_string())
    })?;

    let mut timestamp: Option<&str> = None;
    let mut signatures = Vec::new();
    for part in signature_header.split(',') {
        let mut pair = part.splitn(2, '=');
        let key = pair.next().unwrap_or_default();
        let value = pair.next().unwrap_or_default();
        match key {
            "t" => timestamp = Some(value),
            "v1" => signatures.push(value),
            _ => {}
        }
    }

    let timestamp = timestamp.ok_or_else(|| {
        CustomError::FaultySetup("Stripe webhook signature is missing a timestamp".to_string())
    })?;

    let signed_payload = format!("{timestamp}.{}", String::from_utf8_lossy(payload));
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|err| CustomError::FaultySetup(format!("invalid webhook secret: {err}")))?;
    mac.update(signed_payload.as_bytes());
    let expected = mac.finalize().into_bytes();

    let matched = signatures.iter().any(|signature| {
        hex::decode(signature)
            .map(|candidate| candidate == expected.as_slice())
            .unwrap_or(false)
    });

    if !matched {
        return Err(CustomError::FaultySetup(
            "Stripe webhook signature verification failed".to_string(),
        ));
    }

    serde_json::from_slice(payload)
        .map_err(|err| CustomError::FaultySetup(format!("invalid Stripe webhook payload: {err}")))
}

fn stripe_secret_key(config: &Config) -> Result<&str, CustomError> {
    config
        .stripe_secret_key
        .as_deref()
        .ok_or_else(|| CustomError::FaultySetup("STRIPE_SECRET_KEY is not configured".to_string()))
}

async fn parse_stripe_response<T: for<'de> Deserialize<'de>>(
    response: reqwest::Response,
    action: &str,
) -> Result<T, CustomError> {
    let status = response.status();
    let body = response.text().await.map_err(|err| {
        CustomError::FaultySetup(format!("failed to read Stripe response: {err}"))
    })?;

    if !status.is_success() {
        return Err(CustomError::FaultySetup(format!(
            "failed to {action}: Stripe returned {status} with body {body}"
        )));
    }

    serde_json::from_str(&body).map_err(|err| {
        CustomError::FaultySetup(format!(
            "failed to decode Stripe response for {action}: {err}"
        ))
    })
}
