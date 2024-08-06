use std::sync::Arc;

use crate::{AzureKeyCredential, CreateChatCompletionsRequest, CreateChatCompletionsResponse, CreateTranscriptionRequest};
use azure_core::{HttpClient, Method, MyForm, Result, Error, Url};
use futures::Stream;
use futures::stream::TryStreamExt;

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

    pub async fn create_chat_completions(&self, deployment_name: &str, api_version: AzureServiceVersion,
        chat_completions_request: &CreateChatCompletionsRequest) 
    -> Result<CreateChatCompletionsResponse> {
        let url = Url::parse(&format!("{}/openai/deployments/{}/chat/completions?api-version={}", 
            &self.endpoint,
            deployment_name,
            api_version.as_str())
        )?;
        let request  = super::build_request(&self.key_credential, url, Method::Post, chat_completions_request)?;
        let response = self.http_client.execute_request(&request).await?;
        response.json::<CreateChatCompletionsResponse>().await
    }    
    
    pub async fn stream_chat_completion(&self, deployment_name: &str, api_version: AzureServiceVersion,
        chat_completions_request: &CreateChatCompletionsRequest) 
    -> Result<impl Stream<Item = Result<String>>> {
        let url = Url::parse(&format!("{}/openai/deployments/{}/chat/completions?api-version={}", 
            &self.endpoint,
            deployment_name,
            api_version.as_str())
        )?;
        let request  = super::build_request(&self.key_credential, url, Method::Post, chat_completions_request)?;
        let response = self.http_client.execute_request(&request).await?;

        Ok(response.into_body()
                .and_then(|chunk| {
                    std::future::ready(std::str::from_utf8(&chunk)
                        .map(String::from)
                        .map_err(Error::from)
                    )
                }
            ))
    }


    pub async fn create_speech_transcription(&self, deployment_name: &str, api_version: AzureServiceVersion,
        create_transcription_request: &CreateTranscriptionRequest) 
    -> Result<String> {
        let url = Url::parse(&format!("{}/openai/deployments/{}/audio/transcriptions?api-version={}", 
            &self.endpoint,
            deployment_name,
            api_version.as_str())
        )?;

        let request = super::build_multipart_request(&self.key_credential, url, || {
            Ok(MyForm::new()
                .text("response_format", create_transcription_request.response_format.to_string())
                .file(create_transcription_request.file_name.clone(), create_transcription_request.file.clone()))
        });

        let response = self.http_client.execute_request(&request?).await?;
        Ok(response.into_body().collect_string().await?)
    }
}

pub enum AzureServiceVersion {
    V2023_09_01Preview,
    V2023_12_01Preview,
}


impl AzureServiceVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            AzureServiceVersion::V2023_09_01Preview => "2023-09-01-preview",
            AzureServiceVersion::V2023_12_01Preview => "2023-12-01-preview",
        }
    }
}
