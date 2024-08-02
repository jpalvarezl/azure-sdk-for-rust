use std::fs::File;
use std::io::Read;

use azure_openai_inference::{AzureServiceVersion, CreateTranscriptionRequest};
use azure_openai_inference::{AzureOpenAIClient, AzureKeyCredential};
use env_logger::Env;

#[tokio::main]
async fn main() {
    // use `RUST_LOG=reqwest=trace,hyper=trace cargo run --example speech_transcription_azure` to get request traces
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").expect("Set AZURE_OPENAI_ENDPOINT env variable");
    let secret = std::env::var("AZURE_OPENAI_KEY").expect("Set AZURE_OPENAI_KEY env variable");

    let openai_client = AzureOpenAIClient::new(endpoint, AzureKeyCredential::new(secret));


    let mut file = File::open("./sdk/openai_inference/assets/JP_it_is_rainy_today.wav").expect("File not found");
    let mut file_contents = Vec::new();
    let _ = file.read_to_end(&mut file_contents).expect("Failed to read file");

    let create_transcription_request = CreateTranscriptionRequest::new_as_text(file_contents, "batman.wav");
    let response = openai_client.create_speech_transcription(
        "whisper-deployment",
        AzureServiceVersion::V2023_12_01Preview,
        &create_transcription_request
    ).await;

    match response {
        Ok(transcription) => {
            println!("{:#?}", &transcription);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
