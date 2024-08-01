use std::sync::Arc;

use crate::{AzureKeyCredential, CreateChatCompletionsRequest, CreateChatCompletionsResponse};
use azure_core::{HttpClient, Method, Result, Url};

pub struct AzureOpenAIClient {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    key_credential: AzureKeyCredential,
}

impl AzureOpenAIClient {

    pub fn new(endpoint: String, key_credential: AzureKeyCredential) -> Self {
        Self {
            http_client: azure_core::new_http_client(),
            endpoint,
            key_credential,
        }
    }

    pub async fn create_chat_completions(&self, deployment_name: &str, api_version: &str,
        chat_completions_request: &CreateChatCompletionsRequest) 
    -> Result<CreateChatCompletionsResponse> {
        let url = Url::parse(&format!("{}/openai/deployments/{}/chat/completions?api-version={}", 
            &self.endpoint,
            deployment_name,
            api_version)
        )?;
        let request  = super::build_request(&self.key_credential, url, Method::Post, chat_completions_request)?;
        let response = self.http_client.execute_request(&request).await?;
        response.json::<CreateChatCompletionsResponse>().await
    }
}
