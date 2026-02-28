use crate::{bitcoin_price, fetch_url};

use monty::{MontyRun, NoLimitTracker, PrintWriter, RunProgress};
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, warn};

#[derive(Deserialize)]
pub struct RunPythonArgs {
    code: String,
}

#[derive(Debug, thiserror::Error)]
#[error("python execution failed: {0}")]
pub struct RunPythonError(String);

#[derive(Deserialize, Serialize)]
pub struct RunPython;

impl Tool for RunPython {
    const NAME: &'static str = "run_python";
    type Error = RunPythonError;
    type Args = RunPythonArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "run_python".to_string(),
            description: "Run a small snippet of sandboxed Python with Monty and return the result. Use this for calculation, looping, or data reshaping. Python code may call bitcoin_price(currency) which returns a fake numeric price, and fetch_url(url) for HTTP(S) GET requests; fetch_url(url) returns the response body as text, so parse JSON with json.loads(fetch_url(url)).".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "Python code to execute. The last expression becomes the result. Host-provided functions include bitcoin_price(currency) and fetch_url(url)."
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
        let runner = MontyRun::new(
            args.code,
            "tool.py",
            vec![],
            vec!["bitcoin_price".to_owned(), "fetch_url".to_owned()],
        )
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
                    "bitcoin_price" => {
                        progress = bitcoin_price::handle_bitcoin_price_call(&args, &kwargs, state)
                            .map_err(|err| {
                                warn!(error = %err, "python tool execution failed after bitcoin_price");
                                RunPythonError(err.to_string())
                            })?;
                    }
                    "fetch_url" => {
                        progress = fetch_url::handle_fetch_url_call(&args, &kwargs, state)
                            .await
                            .map_err(|err| {
                                warn!(error = %err, "python tool execution failed after fetch_url");
                                RunPythonError(err.to_string())
                            })?;
                    }
                    _ => {
                        warn!(function_name = %function_name, "python tool called unsupported external function");
                        return Err(RunPythonError(format!(
                            "unsupported external function: {function_name}"
                        )));
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
