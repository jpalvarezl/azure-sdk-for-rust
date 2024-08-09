use azure_core::Result;

use azure_openai_inference::{AzureKeyCredential, AzureOpenAIClient};
use azure_openai_inference::{AzureServiceVersion, CreateChatCompletionsRequest};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint =
        std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let openai_client = AzureOpenAIClient::new(endpoint, AzureKeyCredential::new(secret));

    let chat_completions_request = CreateChatCompletionsRequest::new_stream_with_user_message(
        "gpt-4-1106-preview",
        "Write me an essay that is at least 200 words long on the nutritional values (or lack thereof) of fast food.
        Start the essay by stating 'this essay will be x many words long' where x is the number of words in the essay.",);

    println!("{:#?}", &chat_completions_request);
    println!("{:#?}", serde_json::to_string(&chat_completions_request));
    let response = openai_client
        .stream_chat_completion(
            &chat_completions_request.model,
            AzureServiceVersion::V2023_12_01Preview,
            &chat_completions_request,
        )
        .await?;

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
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    Ok(())
}
