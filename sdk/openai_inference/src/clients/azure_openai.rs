use crate::CreateChatCompletionsRequest;

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

    pub fn create_chat_completions(chat_completions_request: &CreateChatCompletionsRequest) -> String {
        format!("AzureOpenAIClient.get_chat_completions {:#?}", chat_completions_request.messages.get(0))
    }
}
