use std::borrow::Borrow;
use std::pin::Pin;

use async_trait::async_trait;
use azure_core::ResponseBody;
use azure_core::{Error, Result};
use futures::{Stream, TryFutureExt};
use futures::StreamExt;
use futures::TryStreamExt;
use tracing::debug;

use crate::CreateChatCompletionsStreamResponse;

#[async_trait::async_trait]
pub trait EventStreamer<T> {
    // read more on Higher-Rank Trait Bounds (HRTBs)
    async fn event_stream<'a>(
        &self,
        mut response_body: ResponseBody,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<T>> + 'a>>>
    where
        T: serde::de::DeserializeOwned + 'a;
}

// there will be polymorphic streams where the along with a "data:" payload, there will be an "event:" payload
// implying a per-event deserialization type. Customer consumption needs to be as seemless as much as as possible.
pub struct ChatCompletionStreamHandler {
    pub(crate) stream_event_delimiter: String,
}

impl ChatCompletionStreamHandler {

    pub fn new(stream_event_delimiter: impl Into<String>) -> Self {
        ChatCompletionStreamHandler {
            stream_event_delimiter: stream_event_delimiter.into()
        }
    }
}

#[async_trait::async_trait]
impl EventStreamer<CreateChatCompletionsStreamResponse> for ChatCompletionStreamHandler {
    async fn event_stream<'a>(
        &self,
        mut response_body: ResponseBody,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<CreateChatCompletionsStreamResponse>> + 'a>>> {

        let stream = string_chunks(response_body, &self.stream_event_delimiter).await?
            .map_ok(|event| {
                // println!("{:?}", &event);
                // serde_json::from_str::<CreateChatCompletionsStreamResponse>(&event).expect("Deserialization failed")
                CreateChatCompletionsStreamResponse { choices: vec![] }
            });
        Ok(Box::pin(stream))
    }
}

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
                // it's + 4 because the \n\n are escaped and represented as [92, 110, 92, 110]
                if let Some(pos) = chunk_buffer.windows(4).position(|window| window == b"\\n\\n") {
                    // the range must include the delimiter bytes
                    let mut bytes = chunk_buffer.drain(..pos + 4).collect::<Vec<_>>();
                    bytes.truncate(bytes.len() - 4);
                    return if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        // We strip the "data: " portion of the event. The rest is always JSON and will be deserialized
                        // by a subsquent mapping function for this stream
                        let yielded_value = yielded_value.trim_start_matches("data:").trim();
                        if (yielded_value == "[DONE]") {
                            return None;
                        } else {
                            Some((Ok(yielded_value.to_string()), (response_body, chunk_buffer)))
                        }
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
                // it's + 4 because the \n\n are escaped and represented as [92, 110, 92, 110]
                if let Some(pos) = chunk_buffer.windows(4).position(|window| window == b"\\n\\n") {
                    // the range must include the delimiter bytes
                    let mut bytes = chunk_buffer.drain(..pos).collect::<Vec<_>>();
                    bytes.truncate(bytes.len() - 4);
                    return if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        let yielded_value = yielded_value.trim_start_matches("data:").trim();
                        if (yielded_value == "[DONE]") {
                            return None;
                        } else {
                            Some((Ok(yielded_value.to_string()), (response_body, chunk_buffer)))
                        }
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

#[cfg(test)]
mod tests {
    use crate::clients::tests::*;

    use super::*;
    use azure_core::ResponseBody;
    use tracing::debug;

    #[tokio::test]
    async fn clean_chunks() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(b"data: piece 1\\n\\n")),
            Ok(bytes::Bytes::from_static(b"data: piece 2\\n\\n")),
            Ok(bytes::Bytes::from_static(b"data: [DONE]\\n\\n")),
        ]);

        let actual = string_chunks(&mut source_stream, "\\n\\n").await?;
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> =
            vec![Ok("piece 1".to_string()), Ok("piece 2".to_string())];
        assert_eq!(expected, actual);

        Ok(())
    }

    #[tokio::test]
    async fn multiple_message_in_one_chunk() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(b"data: piece 1\\n\\ndata: piece 2\\n\\n")),
            Ok(bytes::Bytes::from_static(b"data: piece 3\\n\\ndata: [DONE]\\n\\n")),
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
            Ok(bytes::Bytes::from_static(b"data: piece 1\\n")),
            Ok(bytes::Bytes::from_static(b"\\ndata: [DONE]")),
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
            Ok(bytes::Bytes::from_static(b"data: piece 1")),
            Ok(bytes::Bytes::from_static(b"\\n\\ndata: [DONE]")),
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
    async fn real_data() -> Result<()> {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from(STREAM_CHUNK_01)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_02)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_03)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_04)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_05)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_06)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_07)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_08)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_09)),
            // Ok(bytes::Bytes::from(STREAM_CHUNK_10)),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n").await?;
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![
            Ok(STREAM_EVENT_01.to_string())
        ];

        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn delimiter_search() -> Result<()> {
        let delimiter = "\\n\\n";
        let data = bytes::Bytes::from(STREAM_CHUNK_01);
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&data);

        // Find the position of the delimiter
        let pos = buffer.windows(4).position(|window| window == delimiter.as_bytes());
        match pos {
            Some(pos) => {
                // it's + 4 because the \n\n are escaped and represented as [92, 110, 92, 110]
                let bytes = buffer.drain(..pos).collect::<Vec<_>>();
                let yielded_value = std::str::from_utf8(&bytes).unwrap();
                let yielded_value = yielded_value.trim_start_matches("data:").trim();

                assert_eq!(yielded_value, STREAM_EVENT_01);
            }
            None => {
                println!("Delimiter not found in the buffer");
                assert!(false, "Delimiter not found");
            }
        }

        Ok(())
    }

}
