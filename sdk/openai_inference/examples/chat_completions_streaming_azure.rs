use azure_core::Result;

use azure_openai_inference::{AzureServiceVersion, CreateChatCompletionsRequest, CreateChatCompletionsStreamResponse};
use azure_openai_inference::{AzureOpenAIClient, AzureKeyCredential};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let openai_client = AzureOpenAIClient::new(endpoint, AzureKeyCredential::new(secret));

    let chat_completions_request = CreateChatCompletionsRequest::new_stream_with_user_message(
        "gpt-4-1106-preview",
        "Write me a 200 words essay on the nutritional values (or lack thereof) of fast food.",);

    println!("{:#?}", &chat_completions_request);
    println!("{:#?}", serde_json::to_string(&chat_completions_request));
    let mut response = openai_client.stream_chat_completion(
        &chat_completions_request.model,
        AzureServiceVersion::V2023_12_01Preview,
        &chat_completions_request
    ).await?;

    // this pins the stream to the stack so it is safe to poll it (namely, it won't be dealloacted or moved)
    futures::pin_mut!(response);

    while let Some(result) = response.next().await {
        match result {
            Ok(delta) => {
                if let Some(choice) = delta.choices.get(0) {
                    choice.delta.as_ref().map(|d| {
                        d.content.as_ref().map(|c| {
                            print!("{}", c);
                        });
                    });
                }
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }

    Ok(())
}
