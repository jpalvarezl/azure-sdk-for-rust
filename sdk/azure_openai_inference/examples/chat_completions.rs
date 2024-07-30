use azure_openai_inference::{AzureOpenAIClient, CreateChatCompletionsOptions};

fn main() {

    let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let key = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let chat_completions = AzureOpenAIClient::get_chat_completions(
        &CreateChatCompletionsOptions::new("Tell me a joke about pineapples")
    );
    println!("Response: {}", chat_completions);
}
