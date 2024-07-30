use azure_openai_inference::AzureOpenAIClient;

fn main() {
    let chat_completions = AzureOpenAIClient::get_chat_completions();
    println!("Response: {}", chat_completions);
}
