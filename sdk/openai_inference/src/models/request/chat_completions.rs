use serde::{Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateChatCompletionsRequest {
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub model: String,
    // pub frequency_penalty: f64,
    // pub logit_bias: Option<HashMap<String, f64>>,
    // pub logprobs: Option<bool>,
    // pub top_logprobs: Option<i32>,
    // pub max_tokens: Option<i32>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct ChatCompletionRequestMessageBase {
    pub name: Option<String>,
    pub content: String, // TODO this should be either a string or ChatCompletionRequestMessageContentPart (a polymorphic type)
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "role")]
pub enum ChatCompletionRequestMessage {
    #[serde(rename = "system")]
    System(ChatCompletionRequestMessageBase),
    #[serde(rename = "user")]
    User(ChatCompletionRequestMessageBase),
}


impl ChatCompletionRequestMessage {
    pub fn new_user(content: &str) -> Self {
        Self::User(ChatCompletionRequestMessageBase {
            content: content.to_string(),
            name: None,
        })
    }

    pub fn new_system(content: &str) -> Self {
        Self::System(ChatCompletionRequestMessageBase {
            content: content.to_string(),
            name: None,
        })
    }
    
}
impl CreateChatCompletionsRequest {

    pub fn new_with_user_message(model: &str, prompt: &str) -> Self {
        Self {
            model: model.to_string(),
            messages: vec![ChatCompletionRequestMessage::new_user(prompt)],
            ..Default::default()
        }
    }
}
