use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use monty::{MontyObject, PrintWriter, ResourceTracker, RunProgress, Snapshot};
use oas3::OpenApiV3Spec;
use oas3::spec::{Operation, ParameterIn, RequestBody, SchemaType, SchemaTypeSet};
use tracing::{info, warn};

#[derive(Clone, Debug)]
pub struct OpenApiRegistry {
    actions: HashMap<String, Arc<OpenApiAction>>,
    plugins: Vec<OpenApiPlugin>,
}

#[derive(Clone, Debug)]
struct OpenApiAction {
    name: String,
    description: String,
    method: String,
    base_url: String,
    path: String,
    parameters: Vec<OpenApiParameter>,
}

#[derive(Clone, Debug)]
struct OpenApiPlugin {
    title: String,
    actions: Vec<String>,
}

#[derive(Clone, Debug)]
struct OpenApiParameter {
    name: String,
    location: ParameterLocation,
    required: bool,
    schema_type: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParameterLocation {
    Path,
    Query,
    Header,
    Body,
}

impl OpenApiRegistry {
    pub fn load_specs_from_dir(path: impl AsRef<Path>) -> anyhow::Result<Vec<OpenApiV3Spec>> {
        let path = path.as_ref();
        let mut specs = Vec::new();

        if !path.exists() {
            return Ok(specs);
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let ext = path.extension().and_then(|ext| ext.to_str());
            if !matches!(ext, Some("yaml" | "yml")) {
                continue;
            }

            let contents = fs::read_to_string(&path)?;
            let spec = oas3::from_yaml(contents)?;
            specs.push(spec);
        }

        Ok(specs)
    }

    pub fn from_specs(specs: &[OpenApiV3Spec]) -> Self {
        let mut actions = HashMap::new();
        let mut plugins = Vec::new();

        for spec in specs {
            let title = spec.info.title.clone();
            let base_url = spec
                .servers
                .first()
                .map(|server| server.url.trim_end_matches('/').to_string())
                .unwrap_or_default();

            let mut plugin_action_names = Vec::new();

            for (path, method, operation) in spec.operations() {
                let Some(operation_id) = operation
                    .operation_id
                    .as_ref()
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
                else {
                    continue;
                };

                let description = operation
                    .description
                    .clone()
                    .or_else(|| operation.summary.clone())
                    .unwrap_or_default();

                let mut parameters = Vec::new();
                collect_operation_parameters(spec, operation, &mut parameters);
                collect_request_body_parameters(spec, operation, &mut parameters);

                let name = operation_id.to_string();
                actions.insert(
                    name.clone(),
                    Arc::new(OpenApiAction {
                        name: name.clone(),
                        description,
                        method: method.as_str().to_string(),
                        base_url: base_url.clone(),
                        path: path.clone(),
                        parameters,
                    }),
                );
                plugin_action_names.push(name);
            }

            plugin_action_names.sort();
            plugin_action_names.dedup();
            if !plugin_action_names.is_empty() {
                plugins.push(OpenApiPlugin {
                    title,
                    actions: plugin_action_names,
                });
            }
        }

        plugins.sort_by(|a, b| a.title.cmp(&b.title));

        Self { actions, plugins }
    }

    pub fn function_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.actions.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn prompt_fragment(&self) -> String {
        if self.plugins.is_empty() {
            return String::new();
        }

        let mut lines = Vec::new();
        for plugin in &self.plugins {
            lines.push(format!("{}:", plugin.title));
            for action_name in &plugin.actions {
                let Some(action) = self.actions.get(action_name) else {
                    continue;
                };
                let args = action
                    .parameters
                    .iter()
                    .map(|param| {
                        let py_type = python_type(&param.schema_type);
                        if param.required {
                            format!("{}: {}", param.name, py_type)
                        } else {
                            format!("{}: {} = None", param.name, py_type)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let signature = format!("{}({}) -> dict | str", action.name, args);
                if action.description.is_empty() {
                    lines.push(format!("- {}", signature));
                } else {
                    lines.push(format!("- {}  # {}", signature, action.description));
                }
            }
        }
        lines.join("\n")
    }

    pub async fn handle_call<T: ResourceTracker>(
        &self,
        function_name: &str,
        args: &[MontyObject],
        kwargs: &[(MontyObject, MontyObject)],
        state: Snapshot<T>,
    ) -> anyhow::Result<RunProgress<T>> {
        let Some(action) = self.actions.get(function_name) else {
            anyhow::bail!("unsupported external function: {function_name}");
        };

        let arg_map = action.bind_args(args, kwargs)?;
        let response = execute_action(action, &arg_map).await?;
        let result = response_to_monty(response);

        let mut writer = PrintWriter::Stdout;
        Ok(state.run(result, &mut writer)?)
    }
}

impl OpenApiAction {
    fn bind_args(
        &self,
        args: &[MontyObject],
        kwargs: &[(MontyObject, MontyObject)],
    ) -> anyhow::Result<HashMap<String, MontyObject>> {
        if args.len() > self.parameters.len() {
            anyhow::bail!("{}() received too many positional arguments", self.name);
        }

        let mut bound = HashMap::new();

        for (param, value) in self.parameters.iter().zip(args.iter()) {
            bound.insert(param.name.clone(), value.clone());
        }

        for (key, value) in kwargs {
            let MontyObject::String(name) = key else {
                anyhow::bail!("{}() keyword names must be strings", self.name);
            };
            if !self.parameters.iter().any(|param| param.name == *name) {
                anyhow::bail!("{}() got unexpected keyword argument '{}'", self.name, name);
            }
            bound.insert(name.clone(), value.clone());
        }

        for param in &self.parameters {
            if param.required && !bound.contains_key(&param.name) {
                anyhow::bail!("{}() missing required argument '{}'", self.name, param.name);
            }
        }

        Ok(bound)
    }
}

fn collect_operation_parameters(
    spec: &OpenApiV3Spec,
    operation: &Operation,
    out: &mut Vec<OpenApiParameter>,
) {
    let Ok(parameters) = operation.parameters(spec) else {
        return;
    };

    for parameter in parameters {
        let location = match parameter.location {
            ParameterIn::Path => ParameterLocation::Path,
            ParameterIn::Query => ParameterLocation::Query,
            ParameterIn::Header => ParameterLocation::Header,
            _ => continue,
        };

        let schema_type = parameter
            .schema
            .as_ref()
            .and_then(|schema| schema.resolve(spec).ok())
            .map(|schema| schema_type(&schema))
            .unwrap_or_else(|| "string".to_string());

        out.push(OpenApiParameter {
            name: parameter.name.clone(),
            location,
            required: parameter.required.unwrap_or(false),
            schema_type,
        });
    }
}

fn collect_request_body_parameters(
    spec: &OpenApiV3Spec,
    operation: &Operation,
    out: &mut Vec<OpenApiParameter>,
) {
    let Ok(Some(request_body)) = operation.request_body(spec) else {
        return;
    };

    collect_request_body_schema(spec, &request_body, out);
}

fn collect_request_body_schema(
    spec: &OpenApiV3Spec,
    request_body: &RequestBody,
    out: &mut Vec<OpenApiParameter>,
) {
    let Some(content) = request_body.content.get("application/json") else {
        return;
    };
    if content.schema.is_none() {
        return;
    }
    let Ok(Some(schema)) = content.schema(spec) else {
        return;
    };

    let required_fields = schema.required.clone();
    let Some(SchemaTypeSet::Single(SchemaType::Object) | SchemaTypeSet::Multiple(_)) =
        &schema.schema_type
    else {
        return;
    };

    for (name, property_schema) in &schema.properties {
        let schema_type = property_schema
            .resolve(spec)
            .map(|schema| schema_type(&schema))
            .unwrap_or_else(|_| "string".to_string());

        out.push(OpenApiParameter {
            name: name.clone(),
            location: ParameterLocation::Body,
            required: required_fields.contains(name),
            schema_type,
        });
    }
}

fn schema_type(schema: &oas3::spec::ObjectSchema) -> String {
    match &schema.schema_type {
        Some(SchemaTypeSet::Single(SchemaType::Integer)) => "integer",
        Some(SchemaTypeSet::Single(SchemaType::Number)) => "number",
        Some(SchemaTypeSet::Single(SchemaType::Boolean)) => "boolean",
        Some(SchemaTypeSet::Single(SchemaType::Array)) => "array",
        Some(SchemaTypeSet::Single(SchemaType::Object)) => "object",
        _ => "string",
    }
    .to_string()
}

fn python_type(schema_type: &str) -> &str {
    match schema_type {
        "integer" => "int",
        "number" => "float",
        "boolean" => "bool",
        "array" => "list",
        "object" => "dict",
        _ => "str",
    }
}

async fn execute_action(
    action: &OpenApiAction,
    args: &HashMap<String, MontyObject>,
) -> anyhow::Result<String> {
    let mut path = action.path.clone();
    let mut query = Vec::<(String, String)>::new();
    let client = reqwest::Client::new();
    let mut headers = Vec::<(String, String)>::new();
    let mut body = serde_json::Map::new();

    for param in &action.parameters {
        let Some(value) = args.get(&param.name) else {
            continue;
        };
        match param.location {
            ParameterLocation::Path => {
                path = path.replace(&format!("{{{}}}", param.name), &monty_to_string(value)?);
            }
            ParameterLocation::Query => query.push((param.name.clone(), monty_to_string(value)?)),
            ParameterLocation::Header => {
                headers.push((param.name.clone(), monty_to_string(value)?))
            }
            ParameterLocation::Body => {
                body.insert(param.name.clone(), monty_to_json(value)?);
            }
        }
    }

    let mut request = client.request(
        reqwest::Method::from_bytes(action.method.as_bytes())?,
        format!("{}{}", action.base_url, path),
    );
    if !query.is_empty() {
        request = request.query(&query);
    }
    for (name, value) in headers {
        request = request.header(name, value);
    }
    if !body.is_empty() {
        request = request.json(&body);
    }

    info!(operation = %action.name, method = %action.method, path = %path, "calling openapi action");
    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        warn!(operation = %action.name, status = %status, bytes = body.len(), "openapi action failed");
        anyhow::bail!("{} failed with status {}: {}", action.name, status, body);
    }
    info!(operation = %action.name, status = %status, bytes = body.len(), "openapi action completed");
    Ok(body)
}

fn monty_to_string(value: &MontyObject) -> anyhow::Result<String> {
    match value {
        MontyObject::String(value) => Ok(value.clone()),
        MontyObject::Int(value) => Ok(value.to_string()),
        MontyObject::Float(value) => Ok(value.to_string()),
        MontyObject::Bool(value) => Ok(value.to_string()),
        MontyObject::None => Ok(String::new()),
        _ => anyhow::bail!("unsupported argument type for string conversion: {value:?}"),
    }
}

fn monty_to_json(value: &MontyObject) -> anyhow::Result<serde_json::Value> {
    match value {
        MontyObject::None => Ok(serde_json::Value::Null),
        MontyObject::Bool(value) => Ok(serde_json::Value::Bool(*value)),
        MontyObject::Int(value) => Ok(serde_json::Value::Number((*value).into())),
        MontyObject::Float(value) => serde_json::Number::from_f64(*value)
            .map(serde_json::Value::Number)
            .ok_or_else(|| anyhow::anyhow!("invalid float value")),
        MontyObject::String(value) => Ok(serde_json::Value::String(value.clone())),
        MontyObject::List(items) | MontyObject::Tuple(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(monty_to_json(item)?);
            }
            Ok(serde_json::Value::Array(out))
        }
        MontyObject::Dict(pairs) => {
            let mut out = serde_json::Map::new();
            for (key, value) in pairs.clone() {
                let MontyObject::String(key) = key else {
                    anyhow::bail!("json body object keys must be strings");
                };
                out.insert(key, monty_to_json(&value)?);
            }
            Ok(serde_json::Value::Object(out))
        }
        _ => anyhow::bail!("unsupported argument type for json conversion: {value:?}"),
    }
}

fn response_to_monty(body: String) -> MontyObject {
    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(value) => json_to_monty(&value),
        Err(_) => MontyObject::String(body),
    }
}

fn json_to_monty(value: &serde_json::Value) -> MontyObject {
    match value {
        serde_json::Value::Null => MontyObject::None,
        serde_json::Value::Bool(value) => MontyObject::Bool(*value),
        serde_json::Value::Number(value) => {
            if let Some(int) = value.as_i64() {
                MontyObject::Int(int)
            } else if let Some(float) = value.as_f64() {
                MontyObject::Float(float)
            } else {
                MontyObject::String(value.to_string())
            }
        }
        serde_json::Value::String(value) => MontyObject::String(value.clone()),
        serde_json::Value::Array(items) => {
            MontyObject::List(items.iter().map(json_to_monty).collect())
        }
        serde_json::Value::Object(map) => MontyObject::dict(
            map.iter()
                .map(|(key, value)| (MontyObject::String(key.clone()), json_to_monty(value)))
                .collect::<Vec<_>>(),
        ),
    }
}
