use azure_core::ResponseBody;

pub trait StreamChunker {
    fn chunk(response_body: &ResponseBody) -> Result<Vec<String>>;
}
