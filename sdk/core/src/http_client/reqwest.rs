use crate::{
    error::{ErrorKind, ResultExt},
    Body, HttpClient, PinnedStream,
};
use async_trait::async_trait;
use futures::TryStreamExt;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tracing::{debug, warn};

/// Construct a new `HttpClient` with the `reqwest` backend.
pub fn new_reqwest_client() -> Arc<dyn HttpClient> {
    debug!("instantiating an http client using the reqwest backend");

    // set `pool_max_idle_per_host` to `0` to avoid an issue in the underlying
    // `hyper` library that causes the `reqwest` client to hang in some cases.
    //
    // See <https://github.com/hyperium/hyper/issues/2312> for more details.
    #[cfg(not(target_arch = "wasm32"))]
    let client = ::reqwest::ClientBuilder::new()
        .connection_verbose(true)
        .pool_max_idle_per_host(0)
        .build()
        .expect("failed to build `reqwest` client");

    // `reqwest` does not implement `pool_max_idle_per_host()` on WASM.
    #[cfg(target_arch = "wasm32")]
    let client = ::reqwest::ClientBuilder::new()
        .build()
        .expect("failed to build `reqwest` client");

    Arc::new(client)
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HttpClient for ::reqwest::Client {
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response> {
        let url = request.url().clone();
        let method = request.method();
        let mut req = self.request(try_from_method(*method)?, url.clone());
        for (name, value) in request.headers().iter() {
            req = req.header(name.as_str(), value.as_str());
        }
        let body = request.body().clone();

        let reqwest_request = match body {
            Body::Multipart(form) => req.multipart(super::to_reqwest_form(form)).build(),
            Body::Bytes(bytes) => req.body(bytes).build(),

            // We cannot currently implement `Body::SeekableStream` for WASM
            // because `reqwest::Body::wrap_stream()` is not implemented for WASM.
            #[cfg(not(target_arch = "wasm32"))]
            Body::SeekableStream(seekable_stream) => req
                .body(::reqwest::Body::wrap_stream(seekable_stream))
                .build(),
        }
        .context(ErrorKind::Other, "failed to build `reqwest` request")?;

        debug!("performing request {method} '{url}' with `reqwest`");
        let rsp = self
            .execute(reqwest_request)
            .await
            .context(ErrorKind::Io, "failed to execute `reqwest` request")?;

        let status = rsp.status();
        let headers = to_headers(rsp.headers());

        let body: PinnedStream = Box::pin(rsp.bytes_stream().map_err(|error| {
            crate::error::Error::full(
                ErrorKind::Io,
                error,
                "error converting `reqwest` request into a byte stream",
            )
        }));

        Ok(crate::Response::new(
            try_from_status(status)?,
            headers,
            body,
        ))
    }
}

fn to_headers(map: &::reqwest::header::HeaderMap) -> crate::headers::Headers {
    let map = map
        .iter()
        .filter_map(|(k, v)| {
            let key = k.as_str();
            if let Ok(value) = v.to_str() {
                Some((
                    crate::headers::HeaderName::from(key.to_owned()),
                    crate::headers::HeaderValue::from(value.to_owned()),
                ))
            } else {
                warn!("header value for `{key}` is not utf8");
                None
            }
        })
        .collect::<HashMap<_, _>>();
    crate::headers::Headers::from(map)
}

fn try_from_method(method: crate::Method) -> crate::Result<::reqwest::Method> {
    match method {
        crate::Method::Connect => Ok(::reqwest::Method::CONNECT),
        crate::Method::Delete => Ok(::reqwest::Method::DELETE),
        crate::Method::Get => Ok(::reqwest::Method::GET),
        crate::Method::Head => Ok(::reqwest::Method::HEAD),
        crate::Method::Options => Ok(::reqwest::Method::OPTIONS),
        crate::Method::Patch => Ok(::reqwest::Method::PATCH),
        crate::Method::Post => Ok(::reqwest::Method::POST),
        crate::Method::Put => Ok(::reqwest::Method::PUT),
        crate::Method::Trace => Ok(::reqwest::Method::TRACE),
        _ => ::reqwest::Method::from_str(method.as_ref()).map_kind(ErrorKind::DataConversion),
    }
}

fn try_from_status(status: ::reqwest::StatusCode) -> crate::Result<crate::StatusCode> {
    let status = u16::from(status);
    crate::StatusCode::try_from(status).map_err(|_| {
        crate::error::Error::with_message(ErrorKind::DataConversion, || {
            format!("invalid status code {status}")
        })
    })
}
