use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChatCompletionsResponse {
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChoice {
    pub message: ChatCompletionResponseMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponseMessage {
    pub content: String,
    pub role: String,
}
