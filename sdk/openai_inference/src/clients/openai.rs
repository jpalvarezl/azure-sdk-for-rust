use std::fmt::format;
use std::ops::Deref;
use std::sync::Arc;
use serde::Serialize;

use azure_core::auth::TokenCredential;
use azure_core::headers::{HeaderName, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use azure_core::{HttpClient, Method, Request, Url};

use crate::auth::*;
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

    pub async fn create_chat_completions(&self, chat_completions_request: &CreateChatCompletionsRequest) -> azure_core::Result<CreateChatCompletionsResponse> {
        let http_client = azure_core::new_http_client();
        let url = Url::parse("https://api.openai.com/v1/chat/completions")?;
        let request  = &self.build_request(url, Method::Post, chat_completions_request);

        let response = self.http_client.execute_request(&request).await?;

        response.json::<CreateChatCompletionsResponse>().await
    }

    fn build_request<T>(&self, url: Url, method: Method, data: &T) -> Request
    where T: ?Sized + Serialize {
        let mut request = Request::new(url, method);
        request.add_mandatory_header(&self.key_credential);
        request.insert_header(CONTENT_TYPE, "application/json");
        request.insert_header(ACCEPT, "application/json");
        request.set_json(data);
        request
    }
}

