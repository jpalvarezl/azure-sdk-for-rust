#[cfg(not(target_arch = "wasm32"))]
use crate::SeekableStream;
use crate::{
    headers::{AsHeaders, Headers},
    to_json, Method, Url,
};
use bytes::Bytes;
use serde::Serialize;
use std::fmt::Debug;

/// An HTTP Body.
#[derive(Debug, Clone)]
pub enum Body {
    Multipart(MyForm),
    /// A body of a known size.
    Bytes(bytes::Bytes),
    /// A streaming body.
    /// This is not currently supported on WASM targets.
    // We cannot currently implement `Body::SeekableStream` for WASM
    // because `reqwest::Body::wrap_stream()` is not implemented for WASM.
    #[cfg(not(target_arch = "wasm32"))]
    SeekableStream(Box<dyn SeekableStream>),
}

impl Body {
    pub fn len(&self) -> usize {
        match self {
            Body::Multipart(_) => 0,
            Body::Bytes(bytes) => bytes.len(),
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(stream) => stream.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) async fn reset(&mut self) -> crate::Result<()> {
        match self {
            Body::Multipart(_) => Ok(()),
            Body::Bytes(_) => Ok(()),
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(stream) => stream.reset().await,
        }
    }
}

impl<B> From<B> for Body
where
    B: Into<Bytes>,
{
    fn from(bytes: B) -> Self {
        Self::Bytes(bytes.into())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Box<dyn SeekableStream>> for Body {
    fn from(seekable_stream: Box<dyn SeekableStream>) -> Self {
        Self::SeekableStream(seekable_stream)
    }
}

impl From<MyForm> for Body {
    fn from(my_form: MyForm) -> Self {
        Self::Multipart(my_form)
    }
}

/// A pipeline request.
///
/// A pipeline request is composed by a destination (uri), a method, a collection of headers and a
/// body. Policies are expected to enrich the request by mutating it.
#[derive(Debug, Clone)]
pub struct Request {
    pub(crate) url: Url,
    pub(crate) method: Method,
    pub(crate) headers: Headers,
    pub(crate) body: Body,
}

impl Request {
    /// Create a new request with an empty body and no headers
    pub fn new(url: Url, method: Method) -> Self {
        Self {
            url,
            method,
            headers: Headers::new(),
            body: Body::Bytes(bytes::Bytes::new()),
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    pub fn path_and_query(&self) -> String {
        let mut result = self.url.path().to_owned();
        if let Some(query) = self.url.query() {
            result.push('?');
            result.push_str(query);
        }
        result
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn insert_headers<T: AsHeaders>(&mut self, headers: &T) {
        for (name, value) in headers.as_headers() {
            self.insert_header(name, value);
        }
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_json<T>(&mut self, data: &T) -> crate::Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.set_body(to_json(data)?);
        Ok(())
    }

    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
    }

    pub fn multipart(&mut self, form: MyForm) {
        self.body = Body::Multipart(form);
    }

    pub fn insert_header<K, V>(&mut self, key: K, value: V)
    where
        K: Into<crate::headers::HeaderName>,
        V: Into<crate::headers::HeaderValue>,
    {
        self.headers.insert(key, value);
    }

    pub fn add_optional_header<T: crate::Header>(&mut self, item: &Option<T>) {
        if let Some(item) = item {
            self.insert_header(item.name(), item.value());
        }
    }

    pub fn add_mandatory_header<T: crate::Header>(&mut self, item: &T) {
        self.insert_header(item.name(), item.value());
    }
}

// Had to add this type because reqwest::multipart::Form does not implement Clone
// reqwest seems to handle the calculation of the content-size, so we don't need to keep
// track of that here. In a proper implementation, we might need to handle it.
#[derive(Debug, Clone)]
pub struct MyForm {
    pub(crate) parts: Vec<MyPart>,
}

impl MyForm {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn text(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.parts.push(MyPart::Text {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    pub fn file(mut self, name: impl Into<String>, bytes: Vec<u8>) -> Self {
        self.parts.push(MyPart::File {
            name: name.into(),
            bytes: bytes,
        });
        self
    }
}

#[derive(Debug, Clone)]
pub enum MyPart {
    Text{name: String, value: String},
    File{name: String, bytes: Vec<u8>},
}

pub(crate) fn to_reqwest_form(form: MyForm) -> reqwest::multipart::Form {
    let mut reqwest_form = reqwest::multipart::Form::new();
    for part in form.parts {
        match part {
            MyPart::Text { name, value } => {
                reqwest_form = reqwest_form.text(name, value);
            }
            // "part name" is no the same as `file_name`. Learned the hard way...
            MyPart::File { name, bytes } => {
                reqwest_form = reqwest_form.part("file",  
                    reqwest::multipart::Part::bytes(bytes).
                    mime_str("application/octet-stream").unwrap()
                    .file_name(name)
                );
            }
        }
    }
    reqwest_form
}
