#[derive(Debug, Clone, Default)]
pub struct CreateTranscriptionRequest {
    pub file: Vec<u8>,
    pub file_name: String,
    pub response_format: OutputFormat,
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateTranslationRequest {
    pub file: Vec<u8>,
    pub file_name: String,
    pub response_format: OutputFormat,
}

#[derive(Debug, Clone, Default)]
pub enum OutputFormat {
    JSON,
    #[default]
    Text,
    SRT,
    VerboseJSON,
    VTT,
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        match self {
            OutputFormat::JSON => "json".to_string(),
            OutputFormat::Text => "text".to_string(),
            OutputFormat::SRT => "srt".to_string(),
            OutputFormat::VerboseJSON => "verbose_json".to_string(),
            OutputFormat::VTT => "vtt".to_string(),
        }
    }
}

impl CreateTranscriptionRequest {
    pub fn new_as_text(file: Vec<u8>, file_name: impl Into<String>) -> Self {
        Self {
            file,
            file_name: file_name.into(),
            model: Some(String::from("whisper-1")), // ignored by azure. TODO: remove. Defaults should be handled better
            ..Default::default()
        }
    }
}
