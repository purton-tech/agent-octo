use serde_json::Value;
use tracing::warn;

pub struct RawSpec {
    pub path: &'static str,
    pub yaml: &'static str,
}

#[derive(Clone, PartialEq, Eq)]
pub struct IntegrationSpec {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub logo_url: Option<String>,
    pub endpoints: Vec<Endpoint>,
    pub yaml: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub method: String,
    pub path: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body_content: Vec<String>,
    pub responses: Vec<Response>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub location: Option<String>,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Response {
    pub status: String,
    pub description: Option<String>,
}

struct IntegrationMetadata {
    slug: String,
    title: String,
    description: Option<String>,
    version: Option<String>,
    logo_url: Option<String>,
}

impl IntegrationSpec {
    pub fn detail_path(&self) -> String {
        format!("/open-api-specs/{}/", self.slug)
    }
}

pub fn load_integration_specs() -> Vec<IntegrationSpec> {
    let mut specs = Vec::new();

    for spec in all_specs() {
        let value: Value = match serde_yaml::from_str(spec.yaml) {
            Ok(value) => value,
            Err(err) => {
                warn!("failed to parse integration {}: {}", spec.path, err);
                continue;
            }
        };

        let metadata = match parse_metadata(&value, spec.path) {
            Some(metadata) => metadata,
            None => {
                warn!(
                    "failed to load integration {}: unable to read metadata",
                    spec.path
                );
                continue;
            }
        };

        let mut endpoints = parse_endpoints(&value);
        endpoints.sort_by(|a, b| a.path.cmp(&b.path).then(a.method.cmp(&b.method)));

        specs.push(IntegrationSpec {
            slug: metadata.slug,
            title: metadata.title,
            description: metadata.description,
            version: metadata.version,
            logo_url: metadata.logo_url,
            endpoints,
            yaml: spec.yaml.to_string(),
        });
    }

    specs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    specs
}

fn all_specs() -> Vec<RawSpec> {
    vec![
        RawSpec {
            path: "airtable",
            yaml: include_str!("../open-api-specs/airtable.yaml"),
        },
        RawSpec {
            path: "appollo",
            yaml: include_str!("../open-api-specs/appollo.yaml"),
        },
        RawSpec {
            path: "dropbox",
            yaml: include_str!("../open-api-specs/dropbox.yaml"),
        },
        RawSpec {
            path: "google/calendar",
            yaml: include_str!("../open-api-specs/google/calendar.yaml"),
        },
        RawSpec {
            path: "google/contacts",
            yaml: include_str!("../open-api-specs/google/contacts.yaml"),
        },
        RawSpec {
            path: "google/drive",
            yaml: include_str!("../open-api-specs/google/drive.yaml"),
        },
        RawSpec {
            path: "search/serper-search",
            yaml: include_str!("../open-api-specs/search/serper-search.yaml"),
        },
        RawSpec {
            path: "crypto/blockchain-ticker",
            yaml: include_str!("../open-api-specs/crypto/blockchain-ticker.yaml"),
        },
    ]
}

fn parse_metadata(spec: &Value, fallback_slug: &str) -> Option<IntegrationMetadata> {
    let info = spec.get("info")?.as_object()?;

    let title = info
        .get("title")
        .and_then(|title| title.as_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| fallback_slug.to_string());

    let description = info
        .get("description")
        .and_then(|description| description.as_str())
        .map(ToString::to_string);

    let version = info
        .get("version")
        .and_then(|version| version.as_str())
        .map(ToString::to_string);

    let logo_url = info
        .get("x-logo")
        .and_then(|logo| match logo {
            Value::Object(obj) => obj.get("url").and_then(|url| url.as_str()),
            Value::String(url) => Some(url.as_str()),
            _ => None,
        })
        .map(ToString::to_string);

    let slug = info
        .get("x-bionic-slug")
        .and_then(|slug| slug.as_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| slugify(fallback_slug));

    Some(IntegrationMetadata {
        slug,
        title,
        description,
        version,
        logo_url,
    })
}

fn parse_endpoints(spec: &Value) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();

    let paths = match spec.get("paths").and_then(|paths| paths.as_object()) {
        Some(paths) => paths,
        None => return endpoints,
    };

    let methods = ["get", "post", "put", "delete", "patch", "options", "head"];

    for (path, path_item) in paths {
        if let Some(path_obj) = path_item.as_object() {
            for method in methods {
                if let Some(operation) = path_obj.get(method).and_then(|value| value.as_object()) {
                    endpoints.push(Endpoint {
                        method: method.to_uppercase(),
                        path: path.clone(),
                        summary: operation
                            .get("summary")
                            .and_then(|summary| summary.as_str())
                            .map(ToString::to_string),
                        description: operation
                            .get("description")
                            .and_then(|description| description.as_str())
                            .map(ToString::to_string),
                        operation_id: operation
                            .get("operationId")
                            .and_then(|id| id.as_str())
                            .map(ToString::to_string),
                        parameters: parse_parameters(operation),
                        request_body_content: parse_request_body(operation),
                        responses: parse_responses(operation),
                    });
                }
            }
        }
    }

    endpoints
}

fn parse_parameters(operation: &serde_json::Map<String, Value>) -> Vec<Parameter> {
    let mut parameters = Vec::new();

    if let Some(array) = operation
        .get("parameters")
        .and_then(|params| params.as_array())
    {
        for param in array {
            if let Some(param_obj) = param.as_object() {
                parameters.push(Parameter {
                    name: param_obj
                        .get("name")
                        .and_then(|name| name.as_str())
                        .unwrap_or("Unnamed parameter")
                        .to_string(),
                    location: param_obj
                        .get("in")
                        .and_then(|location| location.as_str())
                        .map(ToString::to_string),
                    required: param_obj
                        .get("required")
                        .and_then(|required| required.as_bool())
                        .unwrap_or(false),
                    description: param_obj
                        .get("description")
                        .and_then(|description| description.as_str())
                        .map(ToString::to_string),
                });
            }
        }
    }

    parameters
}

fn parse_request_body(operation: &serde_json::Map<String, Value>) -> Vec<String> {
    let mut content_types = Vec::new();

    if let Some(body) = operation
        .get("requestBody")
        .and_then(|body| body.as_object())
    {
        if let Some(content) = body.get("content").and_then(|content| content.as_object()) {
            for content_type in content.keys() {
                content_types.push(content_type.to_string());
            }
        }
    }

    content_types.sort();
    content_types
}

fn parse_responses(operation: &serde_json::Map<String, Value>) -> Vec<Response> {
    let mut responses = Vec::new();

    if let Some(response_map) = operation
        .get("responses")
        .and_then(|responses| responses.as_object())
    {
        for (status, response) in response_map {
            if let Some(response_obj) = response.as_object() {
                responses.push(Response {
                    status: status.to_string(),
                    description: response_obj
                        .get("description")
                        .and_then(|description| description.as_str())
                        .map(ToString::to_string),
                });
            } else {
                responses.push(Response {
                    status: status.to_string(),
                    description: None,
                });
            }
        }
    }

    responses.sort_by(|a, b| a.status.cmp(&b.status));
    responses
}

fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | '0'..='9' => ch,
            'A'..='Z' => ch.to_ascii_lowercase(),
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
