use azure_openai_inference::{AzureServiceVersion, CreateChatCompletionsRequest};
use azure_openai_inference::{AzureOpenAIClient, AzureKeyCredential};

#[tokio::main]
async fn main() {
    let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let openai_client = AzureOpenAIClient::new(endpoint, AzureKeyCredential::new(secret));

    let chat_completions_request = CreateChatCompletionsRequest::new_stream_with_user_message(
        "gpt-4-1106-preview",
        "Write me a 1000 words essay on the nutritional values (or lack thereof) of fast food.",);

    println!("{:#?}", &chat_completions_request);
    println!("{:#?}", serde_json::to_string(&chat_completions_request));
    let response = openai_client.stream_chat_completion(
        &chat_completions_request.model,
        AzureServiceVersion::V2023_12_01Preview,
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
