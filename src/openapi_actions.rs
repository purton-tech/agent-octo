use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use monty::{MontyObject, PrintWriter, ResourceTracker, RunProgress, Snapshot};
use serde_json::Value;
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
    pub fn load_specs_from_dir(path: impl AsRef<Path>) -> anyhow::Result<Vec<Value>> {
        let mut specs = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            let contents = fs::read_to_string(&path)?;
            let spec: Value = serde_json::from_str(&contents)?;
            specs.push(spec);
        }

        Ok(specs)
    }

    pub fn from_specs(specs: &[Value]) -> Self {
        let mut actions = HashMap::new();
        let mut plugins = Vec::new();

        for spec in specs {
            let info = spec.get("info").and_then(Value::as_object);
            let title = info
                .and_then(|info| info.get("title"))
                .and_then(Value::as_str)
                .unwrap_or("OpenAPI Plugin")
                .to_string();
            let base_url = spec
                .get("servers")
                .and_then(Value::as_array)
                .and_then(|servers| servers.first())
                .and_then(|server| server.get("url"))
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim_end_matches('/')
                .to_string();

            let Some(paths) = spec.get("paths").and_then(Value::as_object) else {
                continue;
            };

            let mut plugin_action_names = Vec::new();

            for (path, path_item) in paths {
                let Some(path_item_obj) = path_item.as_object() else {
                    continue;
                };

                for method in ["get", "post", "put", "patch", "delete"] {
                    let Some(operation) = path_item_obj.get(method).and_then(Value::as_object)
                    else {
                        continue;
                    };
                    let Some(operation_id) = operation
                        .get("operationId")
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                    else {
                        continue;
                    };

                    let description = operation
                        .get("description")
                        .and_then(Value::as_str)
                        .or_else(|| operation.get("summary").and_then(Value::as_str))
                        .unwrap_or("")
                        .to_string();

                    let mut parameters = Vec::new();
                    collect_parameters(path_item_obj.get("parameters"), &mut parameters);
                    collect_parameters(operation.get("parameters"), &mut parameters);
                    collect_request_body(operation.get("requestBody"), &mut parameters);

                    actions.insert(
                        operation_id.to_string(),
                        Arc::new(OpenApiAction {
                            name: operation_id.to_string(),
                            description,
                            method: method.to_uppercase(),
                            base_url: base_url.clone(),
                            path: path.clone(),
                            parameters,
                        }),
                    );
                    plugin_action_names.push(operation_id.to_string());
                }
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

fn collect_parameters(value: Option<&Value>, out: &mut Vec<OpenApiParameter>) {
    let Some(params) = value.and_then(Value::as_array) else {
        return;
    };

    for param in params {
        let Some(param_obj) = param.as_object() else {
            continue;
        };
        let Some(name) = param_obj.get("name").and_then(Value::as_str) else {
            continue;
        };
        let Some(location) = param_obj.get("in").and_then(Value::as_str) else {
            continue;
        };
        let Some(location) = parse_location(location) else {
            continue;
        };
        let schema_type = param_obj
            .get("schema")
            .and_then(|schema| schema.get("type"))
            .and_then(Value::as_str)
            .unwrap_or("string")
            .to_string();
        let required = param_obj
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        out.push(OpenApiParameter {
            name: name.to_string(),
            location,
            required,
            schema_type,
        });
    }
}

fn collect_request_body(value: Option<&Value>, out: &mut Vec<OpenApiParameter>) {
    let Some(request_body) = value.and_then(Value::as_object) else {
        return;
    };
    let required_fields = request_body
        .get("content")
        .and_then(|content| content.get("application/json"))
        .and_then(|item| item.get("schema"))
        .and_then(|schema| schema.get("required"))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let Some(properties) = request_body
        .get("content")
        .and_then(|content| content.get("application/json"))
        .and_then(|item| item.get("schema"))
        .and_then(|schema| schema.get("properties"))
        .and_then(Value::as_object)
    else {
        return;
    };

    for (name, schema) in properties {
        let schema_type = schema
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("string")
            .to_string();
        out.push(OpenApiParameter {
            name: name.clone(),
            location: ParameterLocation::Body,
            required: required_fields.contains(name),
            schema_type,
        });
    }
}

fn parse_location(value: &str) -> Option<ParameterLocation> {
    match value {
        "path" => Some(ParameterLocation::Path),
        "query" => Some(ParameterLocation::Query),
        "header" => Some(ParameterLocation::Header),
        _ => None,
    }
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

fn monty_to_json(value: &MontyObject) -> anyhow::Result<Value> {
    match value {
        MontyObject::None => Ok(Value::Null),
        MontyObject::Bool(value) => Ok(Value::Bool(*value)),
        MontyObject::Int(value) => Ok(Value::Number((*value).into())),
        MontyObject::Float(value) => serde_json::Number::from_f64(*value)
            .map(Value::Number)
            .ok_or_else(|| anyhow::anyhow!("invalid float value")),
        MontyObject::String(value) => Ok(Value::String(value.clone())),
        MontyObject::List(items) | MontyObject::Tuple(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(monty_to_json(item)?);
            }
            Ok(Value::Array(out))
        }
        MontyObject::Dict(pairs) => {
            let mut out = serde_json::Map::new();
            for (key, value) in pairs.clone() {
                let MontyObject::String(key) = key else {
                    anyhow::bail!("json body object keys must be strings");
                };
                out.insert(key, monty_to_json(&value)?);
            }
            Ok(Value::Object(out))
        }
        _ => anyhow::bail!("unsupported argument type for json conversion: {value:?}"),
    }
}

fn response_to_monty(body: String) -> MontyObject {
    match serde_json::from_str::<Value>(&body) {
        Ok(value) => json_to_monty(&value),
        Err(_) => MontyObject::String(body),
    }
}

fn json_to_monty(value: &Value) -> MontyObject {
    match value {
        Value::Null => MontyObject::None,
        Value::Bool(value) => MontyObject::Bool(*value),
        Value::Number(value) => {
            if let Some(int) = value.as_i64() {
                MontyObject::Int(int)
            } else if let Some(float) = value.as_f64() {
                MontyObject::Float(float)
            } else {
                MontyObject::String(value.to_string())
            }
        }
        Value::String(value) => MontyObject::String(value.clone()),
        Value::Array(items) => MontyObject::List(items.iter().map(json_to_monty).collect()),
        Value::Object(map) => MontyObject::dict(
            map.iter()
                .map(|(key, value)| (MontyObject::String(key.clone()), json_to_monty(value)))
                .collect::<Vec<_>>(),
        ),
    }
}
