use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = Client::from_env(); // method provided by the ProviderClient trait

    let agent = openai_client
        .agent("gpt-5-mini") // method provided by CompletionClient trait
        .preamble("You are a helpful assistant. Be very brief and concise")
        .name("Bob") // used in logging
        .build();

    let prompt = "What is the Rust programming language?";
    println!("{prompt}");

    let response_text = agent.prompt(prompt).await?; // prompt method provided by Prompt trait

    println!("Response: {response_text}");

    Ok(())
}
