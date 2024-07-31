use azure_openai_inference::{AzureOpenAIClient, CreateChatCompletionsRequest};

fn main() {

    let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let key = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let chat_completion_request = CreateChatCompletionsRequest::new_with_user_message("Tell me a joke about pineapples");
    let chat_completions = AzureOpenAIClient::create_chat_completions(
        &chat_completion_request
    );

    println!("{:#?}", serde_json::to_string(&chat_completion_request).unwrap());
    println!("Response: {}", chat_completions);
}
