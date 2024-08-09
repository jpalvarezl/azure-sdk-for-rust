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
                // Looking for the next occurence of the event delimiter
                while let Some(pos) = chunk_buffer.windows(2).position(|window| window == b"\n\n") {
                    let mut bytes = chunk_buffer.drain(..pos + 2).collect::<Vec<_>>();
                    // we remove the delimiter
                    bytes.truncate(bytes.len() - 2);
                    return if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        // We strip the "data: " portion of the event. The rest is always JSON and will be deserialized
                        // by a subsquent mapping function for this stream
                        let yielded_value =
                            yielded_value.split(":").collect::<Vec<&str>>()[1].trim();
                        Some((Ok(yielded_value.to_string()), (response_body, chunk_buffer)))
                    } else {
                        None
                    };
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
            // We drain the buffer of any messages that may be left over.
            // The block above will be skipped, since response_body.next() will be None every time
            } else if !chunk_buffer.is_empty() {
                // we need to verify if there are any event left in the buffer and emit them individually
                while let Some(pos) = chunk_buffer.windows(2).position(|window| window == b"\n\n") {
                    let mut bytes = chunk_buffer.drain(..pos + 2).collect::<Vec<_>>();
                    bytes.truncate(bytes.len() - 2);
                    return if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        let yielded_value =
                            yielded_value.split(":").collect::<Vec<&str>>()[1].trim();
                        Some((Ok(yielded_value.to_string()), (response_body, chunk_buffer)))
                    } else {
                        None
                    };
                }
                // if we get to this point, it means we have drained the buffer of all events, meaning that we haven't been able to find the next delimiter
            }
            None
        },
    );

    // We filter errors, we should specifically target the error type yielded when we are not able to find an event in a chunk
    // Specifically the Error::with_messagge(ErrorKind::DataConversion, || "Incomplete chunk")
    return Ok(stream.filter(|it| std::future::ready(it.is_ok())));
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
            Ok(bytes::Bytes::from("data: [DONE]")),
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
            Ok(bytes::Bytes::from("data: piece 3\n\ndata: [DONE]")),
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

    #[tokio::test]
    async fn event_delimeter_split_across_chunks() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from("data: piece 1\n")),
            Ok(bytes::Bytes::from("\ndata: [DONE]")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n").await?;
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![
            Ok("piece 1".to_string()),
        ];
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn event_delimiter_at_start_of_next_chunk() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from("data: piece 1")),
            Ok(bytes::Bytes::from("\n\ndata: [DONE]")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n").await?;
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![
            Ok("piece 1".to_string()),
        ];
        assert_eq!(expected, actual);
        Ok(())
    }
}
