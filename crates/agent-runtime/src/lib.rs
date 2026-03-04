pub mod config;
pub mod provider;
pub mod system_prompt;

use std::collections::HashMap;
use std::sync::Arc;

use rig::client::CompletionClient;
use rig::completion::{Chat, Message as RigMessage};
use rig::providers::openai::Client;
use tokio::sync::RwLock;
use tool_runtime::monty_python::RunPython;
use tool_runtime::openapi_actions::OpenApiRegistry;

const MAX_HISTORY_MESSAGES: usize = 20;

pub fn build_system_prompt(base_prompt: &str, openapi_actions: &OpenApiRegistry) -> String {
    let dynamic_actions = openapi_actions.prompt_fragment();
    if dynamic_actions.is_empty() {
        base_prompt.to_string()
    } else if let Some((before_rule, rule_and_after)) = base_prompt.split_once("\n## Rule\n") {
        format!("{before_rule}\n{dynamic_actions}\n\n## Rule\n{rule_and_after}")
    } else {
        format!("{base_prompt}\n\n{dynamic_actions}")
    }
}

pub fn build_agent(
    client: Client,
    model: &str,
    system_prompt: String,
    openapi_actions: Arc<OpenApiRegistry>,
) -> impl Chat + Clone {
    client
        .agent(model)
        .preamble(&system_prompt)
        .name("Bob")
        .default_max_turns(4)
        .tool(RunPython::new(openapi_actions))
        .build()
}

pub struct ConversationStore {
    inner: RwLock<HashMap<i64, Vec<RigMessage>>>,
}

impl Default for ConversationStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ConversationStore {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub async fn history(&self, chat_id: i64) -> Vec<RigMessage> {
        let histories = self.inner.read().await;
        histories.get(&chat_id).cloned().unwrap_or_default()
    }

    pub async fn push_turn(&self, chat_id: i64, user: &str, assistant: &str) {
        let mut histories = self.inner.write().await;
        let history = histories.entry(chat_id).or_default();
        history.push(RigMessage::user(user));
        history.push(RigMessage::assistant(assistant));
        trim_history(history);
    }
}

fn trim_history(history: &mut Vec<RigMessage>) {
    if history.len() > MAX_HISTORY_MESSAGES {
        let drop_count = history.len() - MAX_HISTORY_MESSAGES;
        history.drain(0..drop_count);
    }
}
