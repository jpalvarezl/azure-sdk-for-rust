use std::borrow::Borrow;
use std::pin::Pin;

use async_trait::async_trait;
use azure_core::ResponseBody;
use azure_core::{Error, Result};
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;

use crate::CreateChatCompletionsStreamResponse;

pub trait EventStreamer<T> {
    // read more on Higher-Rank Trait Bounds (HRTBs)
    fn event_stream<'a>(
        &self,
        response_body: &mut ResponseBody,
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
/// * `response_body` - The response body from an HTTP request. Using a type easy to test but hard to read. This is just a azure_core::ResponseBody
/// * `stream_event_delimiter` - The delimiter of server sent events. Usually either "\n\n" or "\r\n\r\n".
async fn string_chunks(
    response_body: (impl Stream<Item = Result<bytes::Bytes>> + Unpin),
    _stream_event_delimiter: &str, // figure out how to use it in the move
) -> Result<impl Stream<Item = Result<String>>> {
    let chunk_buffer = Vec::new();
    let stream = futures::stream::unfold(
        (response_body, chunk_buffer),
        |(mut response_body, mut chunk_buffer)| async move {
            if let Some(Ok(bytes)) = response_body.next().await {
                chunk_buffer.extend_from_slice(&bytes);
                while let Some(pos) = chunk_buffer.windows(2).position(|window| window == b"\n\n") {
                    let mut bytes = chunk_buffer.drain(..pos + 2).collect::<Vec<_>>();
                    bytes.truncate(bytes.len() - 2);
                    if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        let yielded_value =
                            yielded_value.split(":").collect::<Vec<&str>>()[1].trim();
                        return Some((
                            Ok(yielded_value.to_string()),
                            (response_body, chunk_buffer),
                        ));
                    } else {
                        return None;
                    }
                }
                if chunk_buffer.len() > 0 {
                    return Some((
                        Err(Error::with_message(
                            azure_core::error::ErrorKind::DataConversion,
                            || "Incomplete chunk",
                        )),
                        (response_body, chunk_buffer),
                    ));
                }
            }
            None
        },
    );

    return Ok(stream);
}

impl EventStreamer<CreateChatCompletionsStreamResponse> for ChatCompletionStreamHandler {
    fn event_stream<'a>(
        &self,
        response_body: &mut ResponseBody,
    ) -> Result<Pin<Box<dyn Stream<Item = CreateChatCompletionsStreamResponse> + 'a>>> {
        let stream_event_delimiter = "\n\n";

        Ok(Box::pin(futures::stream::iter(vec![
            CreateChatCompletionsStreamResponse { choices: vec![] },
        ])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::ResponseBody;
    use tracing::debug;

    #[tokio::test]
    async fn clean_chunks() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from("data: piece 1\n\n")),
            Ok(bytes::Bytes::from("data: piece 2\n\n")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n").await?;
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> =
            vec![Ok("piece 1".to_string()), Ok("piece 2".to_string())];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[tokio::test]
    async fn multiple_message_in_one_chunk() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from("data: piece 1\n\ndata: piece 2\n\n")),
            Ok(bytes::Bytes::from("data: piece 3\n\n")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n").await?;
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![
            Ok("piece 1".to_string()),
            Ok("piece 2".to_string()),
            Ok("piece 3".to_string()),
        ];
        assert_eq!(expected, actual);
        Ok(())
    }
}
