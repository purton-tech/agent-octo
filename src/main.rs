use std::sync::Arc;

use monty::{MontyRun, NoLimitTracker, PrintWriter};
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::{Prompt, ToolDefinition};
use rig::providers::openai::Client;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tracing::{info, warn};

#[derive(Deserialize)]
struct RunPythonArgs {
    code: String,
}

#[derive(Debug, thiserror::Error)]
#[error("python execution failed: {0}")]
struct RunPythonError(String);

#[derive(Deserialize, Serialize)]
struct RunPython;

impl Tool for RunPython {
    const NAME: &'static str = "run_python";
    type Error = RunPythonError;
    type Args = RunPythonArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "run_python".to_string(),
            description: "Run a small snippet of sandboxed Python with Monty and return the result. Use this for calculation, looping, or data reshaping.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "Python code to execute. The last expression becomes the result."
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
        let runner = MontyRun::new(args.code, "tool.py", vec![], vec![]).map_err(|err| {
            warn!(error = %err, "failed to initialize python tool");
            RunPythonError(err.to_string())
        })?;
        let mut writer = PrintWriter::Stdout;
        let output = runner
            .run(vec![], NoLimitTracker, &mut writer)
            .map_err(|err| {
                warn!(error = %err, "python tool execution failed");
                RunPythonError(err.to_string())
            })?;
        info!("python tool completed");

        Ok(format!("result: {output:?}"))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("workspace=info"))
        .with_target(false)
        .init();

    let openai_client = Client::from_env();
    let agent = Arc::new(
        openai_client
            .agent("gpt-5-mini") // method provided by CompletionClient trait
            .preamble("You are a helpful assistant. Be very brief and concise. You can use the run_python tool for calculations or short code execution when helpful.")
            .name("Bob") // used in logging
            .tool(RunPython)
            .build(),
    );
    let bot = Bot::new(std::env::var("TELEGRAM_BOT_TOKEN")?);
    info!("telegram bot started");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = Arc::clone(&agent);
        async move {
            let Some(text) = msg.text() else {
                return respond(());
            };
            info!(chat_id = msg.chat.id.0, "received telegram message");
            let _ = bot.send_chat_action(msg.chat.id, ChatAction::Typing).await;

            let reply = match agent.prompt(text).await {
                Ok(reply) => reply,
                Err(err) => format!("Model error: {err}"),
            };

            if reply.starts_with("Model error: ") {
                warn!(chat_id = msg.chat.id.0, error = %reply, "model request failed");
            }
            bot.send_message(msg.chat.id, reply).await?;
            info!(chat_id = msg.chat.id.0, "sent telegram reply");
            respond(())
        }
    })
    .await;

    Ok(())
}
