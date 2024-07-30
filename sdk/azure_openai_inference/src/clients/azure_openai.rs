pub struct AzureOpenAIClient {
    endpoint: String,
    service_version: String,
}

impl AzureOpenAIClient {

    pub fn new(endpoint: String, service_version: String) -> Self {
        Self {
            endpoint,
            service_version,
        }
    }

    pub fn get_chat_completions(&self) -> String {
        String::from("AzureOpenAIClient.get_chat_completions")
    }
}
