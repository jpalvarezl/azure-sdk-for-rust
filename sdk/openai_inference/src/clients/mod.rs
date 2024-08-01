use azure_core::{headers::{ACCEPT, CONTENT_TYPE}, Header, Method, Request};
use serde::Serialize;
use azure_core::{Result, Url};

pub mod openai;
pub mod azure_openai;

pub(crate) fn build_request<T>(
    key_credential: &impl Header, 
    url: Url, 
    method: Method, data: &T) -> Result<Request>
where T: ?Sized + Serialize {
    let mut request = Request::new(url, method);
    request.add_mandatory_header(key_credential);
    request.insert_header(CONTENT_TYPE, "application/json");
    request.insert_header(ACCEPT, "application/json");
    request.set_json(data)?;
    Ok(request)
}
