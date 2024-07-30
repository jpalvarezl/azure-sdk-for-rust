use crate::CreateChatCompletionsOptions;

pub struct AzureOpenAIClient {
    endpoint: String,
    service_version: String,
}

impl AzureOpenAIClient {

    pub fn new(endpoint: String, service_version: String) -> Self {
        Self {
            endpoint,
            service_version,
        }
    }

    pub fn get_chat_completions(chat_completions_options: &CreateChatCompletionsOptions) -> String {
        format!("AzureOpenAIClient.get_chat_completions {}", chat_completions_options.prompt)
    }
}
