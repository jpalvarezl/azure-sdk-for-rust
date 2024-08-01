use crate::{AzureKeyCredential, CreateChatCompletionsRequest};

pub struct AzureOpenAIClient {
    endpoint: String,
    key_credential: AzureKeyCredential,
}

impl AzureOpenAIClient {

    pub fn new(endpoint: String, key_credential: AzureKeyCredential) -> Self {
        Self {
            endpoint,
            key_credential,
        }
    }

    pub fn create_chat_completions(chat_completions_request: &CreateChatCompletionsRequest) -> String {
        format!("AzureOpenAIClient.get_chat_completions {:#?}", chat_completions_request.messages.get(0))
    }
}
