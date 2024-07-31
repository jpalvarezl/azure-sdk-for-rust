use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct CreateChatCompletionsOptions {
    pub prompt:String,

}

impl CreateChatCompletionsOptions {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: String::from(prompt),
        }
    }
}
