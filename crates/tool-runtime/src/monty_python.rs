use std::sync::Arc;

use crate::{fetch_url, openapi_actions::OpenApiRegistry};

use monty::{MontyRun, NoLimitTracker, PrintWriter, RunProgress};
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::Deserialize;
use serde_json::json;
use tracing::{info, warn};

#[derive(Deserialize)]
pub struct RunPythonArgs {
    code: String,
}

#[derive(Debug, thiserror::Error)]
#[error("python execution failed: {0}")]
pub struct RunPythonError(String);

#[derive(Clone)]
pub struct RunPython {
    openapi_actions: Arc<OpenApiRegistry>,
}

impl RunPython {
    pub fn new(openapi_actions: Arc<OpenApiRegistry>) -> Self {
        Self { openapi_actions }
    }
}

impl Tool for RunPython {
    const NAME: &'static str = "run_python";
    type Error = RunPythonError;
    type Args = RunPythonArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "run_python".to_string(),
            description: "Run a small snippet of sandboxed Python with Monty and return the result. Use this for calculation, looping, or data reshaping. Python code may call fetch_url(url) and any dynamically loaded OpenAPI actions.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "Python code to execute. The last expression becomes the result. Host-provided functions include fetch_url(url) and runtime OpenAPI actions."
                    }
                },
                "required": ["code"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> std::result::Result<Self::Output, Self::Error> {
        let code_len = args.code.len();
        let code_preview = args
            .code
            .lines()
            .next()
            .unwrap_or("")
            .chars()
            .take(80)
            .collect::<String>();
        info!(code_len, code_preview = %code_preview, "running python tool");
        let runner = MontyRun::new(args.code, "tool.py", vec![], {
            let mut functions = vec!["fetch_url".to_owned()];
            functions.extend(self.openapi_actions.function_names());
            functions
        })
        .map_err(|err| {
            warn!(error = %err, "failed to initialize python tool");
            RunPythonError(err.to_string())
        })?;
        let mut progress = {
            let mut writer = PrintWriter::Stdout;
            runner.start(vec![], NoLimitTracker, &mut writer)
        }
        .map_err(|err| {
            warn!(error = %err, "python tool execution failed");
            RunPythonError(err.to_string())
        })?;

        loop {
            match progress {
                RunProgress::Complete(output) => {
                    info!("python tool completed");
                    return Ok(format!("result: {output:?}"));
                }
                RunProgress::FunctionCall {
                    function_name,
                    args,
                    kwargs,
                    state,
                    ..
                } => match function_name.as_str() {
                    "fetch_url" => {
                        progress = fetch_url::handle_fetch_url_call(&args, &kwargs, state)
                            .await
                            .map_err(|err| {
                                warn!(error = %err, "python tool execution failed after fetch_url");
                                RunPythonError(err.to_string())
                            })?;
                    }
                    _ => {
                        progress = self
                            .openapi_actions
                            .handle_call(&function_name, &args, &kwargs, state)
                            .await
                            .map_err(|err| {
                                warn!(function_name = %function_name, error = %err, "python tool external function failed");
                                RunPythonError(err.to_string())
                            })?;
                    }
                },
                RunProgress::OsCall { function, .. } => {
                    warn!(function = %function, "python tool blocked os call");
                    return Err(RunPythonError(format!("unsupported os call: {function}")));
                }
                RunProgress::ResolveFutures(_) => {
                    warn!("python tool hit unresolved future");
                    return Err(RunPythonError(
                        "async futures are not supported in this tool".to_string(),
                    ));
                }
            }
        }
    }
}
