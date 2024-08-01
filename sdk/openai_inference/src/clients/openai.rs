use std::sync::Arc;

use azure_core::{HttpClient, Method, Url};

use crate::auth::OpenAIKeyCredential;
use crate::{CreateChatCompletionsRequest, CreateChatCompletionsResponse};

pub struct OpenAIClient {
    http_client: Arc<dyn HttpClient>,
    key_credential: OpenAIKeyCredential, // should this be an Arc? Probably not, we want this live as long as the client
}

impl OpenAIClient {

    pub fn new(key_credential: OpenAIKeyCredential) -> Self {
        Self {
            http_client: azure_core::new_http_client(),
            key_credential,
        }
    }

    pub async fn create_chat_completions(&self, chat_completions_request: &CreateChatCompletionsRequest) 
    -> azure_core::Result<CreateChatCompletionsResponse> {
        let url = Url::parse("https://api.openai.com/v1/chat/completions")?;
        let request  = super::build_request(&self.key_credential, url, Method::Post, chat_completions_request)?;

        let response = self.http_client.execute_request(&request).await?;

        response.json::<CreateChatCompletionsResponse>().await
    }
}

