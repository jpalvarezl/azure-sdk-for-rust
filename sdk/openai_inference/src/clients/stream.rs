use std::pin::Pin;

use async_trait::async_trait;
use azure_core::ResponseBody;
use azure_core::Result;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;

use crate::CreateChatCompletionsStreamResponse;

pub trait EventStreamer<T> {
    // read more on Higher-Rank Trait Bounds (HRTBs)
    fn event_stream<'a>(
        &self,
        response_body: &ResponseBody,
    ) -> Result<Pin<Box<dyn Stream<Item = T> + 'a>>>
    where
        T: serde::de::DeserializeOwned + 'a;
}

// there will be polymorphic streams where the along with a "data:" payload, there will be an "event:" payload
// implying a per-event deserialization type. Customer consumption needs to be as seemless as much as as possible.
pub struct ChatCompletionStreamHandler;

/// This function chunks a response body from an HTTP request. It assumes a UTF8 encoding. The delimiter of chunks
/// can be different on whether it's an Azure endpoint or the unbranded OpenAI service.
///
/// * `response_body` - The response body from an HTTP request.
/// * `stream_event_delimiter` - The delimiter of server sent events. Usually either "\n\n" or "\r\n\r\n".
async fn string_chunks(
    response_body: &ResponseBody,
    stream_event_delimiter: &str,
) -> Result<impl Stream<Item = String>> {
    Ok(futures::stream::iter(vec!["".to_string()]))
}

impl EventStreamer<CreateChatCompletionsStreamResponse> for ChatCompletionStreamHandler {
    fn event_stream<'a>(
        &self,
        response_body: &ResponseBody,
    ) -> Result<Pin<Box<dyn Stream<Item = CreateChatCompletionsStreamResponse> + 'a>>> {
        let stream_event_delimiter = "\n\n";
        Ok(Box::pin(futures::stream::iter(vec![
            CreateChatCompletionsStreamResponse { choices: vec![] },
        ])))
    }
}
