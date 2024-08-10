pub const STREAM_CHUNK_01: &'static str = r#"data: {"choices":[],"created":0,"id":"","model":"","object":"","prompt_filter_results":[{"prompt_index":0,"content_filter_results":{"custom_blocklists":[],"hate":{"filtered":false,"severity":"safe"},"jailbreak":{"filtered":false,"detected":false},"profanity":{"filtered":false,"detected":false},"self_harm":{"filtered":false,"severity":"safe"},"sexual":{"filtered":false,"severity":"safe"},"violence":{"filtered":false,"severity":"safe"}}}]}\n\n"#;
pub const STREAM_EVENT_01: &'static str = r#"{"choices":[],"created":0,"id":"","model":"","object":"","prompt_filter_results":[{"prompt_index":0,"content_filter_results":{"custom_blocklists":[],"hate":{"filtered":false,"severity":"safe"},"jailbreak":{"filtered":false,"detected":false},"profanity":{"filtered":false,"detected":false},"self_harm":{"filtered":false,"severity":"safe"},"sexual":{"filtered":false,"severity":"safe"},"violence":{"filtered":false,"severity":"safe"}}}]}"#;

pub const STREAM_CHUNK_02: &'static str = include_str!("resources/stream_chunk_02.trace");
pub const STREAM_CHUNK_03: &'static str = include_str!("resources/stream_chunk_03.trace");
pub const STREAM_CHUNK_04: &'static str = include_str!("resources/stream_chunk_04.trace");
pub const STREAM_CHUNK_05: &'static str = include_str!("resources/stream_chunk_05.trace");
pub const STREAM_CHUNK_06: &'static str = include_str!("resources/stream_chunk_06.trace");
pub const STREAM_CHUNK_07: &'static str = include_str!("resources/stream_chunk_07.trace");
pub const STREAM_CHUNK_08: &'static str = include_str!("resources/stream_chunk_08.trace");
pub const STREAM_CHUNK_09: &'static str = include_str!("resources/stream_chunk_09.trace");

pub const STREAM_CHUNK_10: &'static str = r#"data: [DONE]\n\n"#;//include_str!("resources/stream_chunk_10.trace");

