use std::fmt::format;
use std::sync::Arc;

use azure_core::auth::TokenCredential;
use azure_core::headers::{HeaderName, AUTHORIZATION, CONTENT_TYPE};
use azure_core::{HttpClient, Method, Request, Url};

use crate::{CreateChatCompletionsRequest, CreateChatCompletionsResponse};

pub struct OpenAIClient {
    secret: String,
    http_client: Arc<dyn HttpClient>,
}

impl OpenAIClient {

    pub fn new(secret: String) -> Self {
        Self {
            secret,
            http_client: azure_core::new_http_client(),
        }
    }

    pub async fn create_chat_completions(&self, chat_completions_request: &CreateChatCompletionsRequest) -> azure_core::Result<CreateChatCompletionsResponse> {
        let http_client = azure_core::new_http_client();
        let url = Url::parse("https://api.openai.com/v1/chat/completions")?;
        let mut request  = Request::new(url, Method::Post);
        request.insert_header(AUTHORIZATION, format!("Bearer {}", &self.secret));
        request.insert_header(CONTENT_TYPE, "application/json");

        println!("{:#?}", request);
        let response = self.http_client.execute_request(&request).await?;
        
        let (status_code, headers, response_body) = response.deconstruct();
        println!("{:#?}", status_code);
        println!("{:#?}", headers);
        println!("{:#?}", response_body);

        response_body.json::<CreateChatCompletionsResponse>().await
    }
}
