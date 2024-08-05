use std::sync::Arc;

use azure_core::{HttpClient, Method, MyForm, Result, Url};

use crate::auth::OpenAIKeyCredential;
use crate::{CreateChatCompletionsRequest, CreateChatCompletionsResponse, CreateTranscriptionRequest};

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
    -> Result<CreateChatCompletionsResponse> {
        let url = Url::parse("https://api.openai.com/v1/chat/completions")?;
        let request  = super::build_request(&self.key_credential, url, Method::Post, chat_completions_request)?;
        let response = self.http_client.execute_request(&request).await?;
        response.json::<CreateChatCompletionsResponse>().await
    }

    pub async fn create_speech_transcription(&self, create_transcription_request: &CreateTranscriptionRequest) 
    -> Result<String> {
        let url = Url::parse(&format!("https://api.openai.com/v1/audio/transcriptions"))?;

        let request = super::build_multipart_request(&self.key_credential, url, || {
            Ok(MyForm::new()
                .text("response_format", create_transcription_request.response_format.to_string())
                .text("model", create_transcription_request.model.as_ref().expect("'model' is required"))
                .file(create_transcription_request.file_name.clone(), create_transcription_request.file.clone()))
        });

        let response = self.http_client.execute_request(&request?).await?;
        Ok(response.into_body().collect_string().await?)
    }
}

