use azure_openai_inference::{CreateChatCompletionsRequest};
use azure_openai_inference::OpenAIClient;

#[tokio::main]
async fn main() {
    let secret = std::env::var("NON_AZURE_OPENAI_KEY=").expect("Set NON_AZURE_OPENAI_KEY= env variable");

    let openai_client = OpenAIClient::new(secret);

    let chat_completions_request = CreateChatCompletionsRequest::new_with_user_message(
        "gpt-4-1106-preview",
        "Tell me a joke about pineapples");

    println!("{:#?}", serde_json::to_string(&chat_completions_request).unwrap());
    let response = openai_client.create_chat_completions(
        &chat_completions_request
    ).await;

    match response {
        Ok(chat_completions) => {
            println!("{:#?}", &chat_completions);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
