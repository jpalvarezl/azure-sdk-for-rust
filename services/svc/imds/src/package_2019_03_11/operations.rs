#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = "https://169.254.169.254/metadata";
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn attested(&self) -> attested::Client {
        attested::Client(self.clone())
    }
    pub fn identity(&self) -> identity::Client {
        identity::Client(self.clone())
    }
    pub fn instances(&self) -> instances::Client {
        instances::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    Instances_GetMetadata(#[from] instances::get_metadata::Error),
    #[error(transparent)]
    Attested_GetDocument(#[from] attested::get_document::Error),
    #[error(transparent)]
    Identity_GetToken(#[from] identity::get_token::Error),
    #[error(transparent)]
    Identity_GetInfo(#[from] identity::get_info::Error),
}
pub mod instances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_metadata(&self, metadata: impl Into<String>) -> get_metadata::Builder {
            get_metadata::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
            }
        }
    }
    pub mod get_metadata {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::Instance, Error>> {
                Box::pin(async move {
                    let url_str = &format!("{}/instance", self.client.endpoint(),);
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2019-03-11");
                    req_builder = req_builder.header("Metadata", &self.metadata);
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::Instance =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod attested {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_document(&self, metadata: impl Into<String>) -> get_document::Builder {
            get_document::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
                nonce: None,
            }
        }
    }
    pub mod get_document {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
            pub(crate) nonce: Option<String>,
        }
        impl Builder {
            pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
                self.nonce = Some(nonce.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::AttestedData, Error>> {
                Box::pin(async move {
                    let url_str = &format!("{}/attested/document", self.client.endpoint(),);
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2019-03-11");
                    if let Some(nonce) = &self.nonce {
                        url.query_pairs_mut().append_pair("nonce", nonce);
                    }
                    req_builder = req_builder.header("Metadata", &self.metadata);
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::AttestedData =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod identity {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_token(&self, metadata: impl Into<String>, resource: impl Into<String>) -> get_token::Builder {
            get_token::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
                resource: resource.into(),
                client_id: None,
                object_id: None,
                msi_res_id: None,
                authority: None,
                bypass_cache: None,
            }
        }
        pub fn get_info(&self, metadata: impl Into<String>) -> get_info::Builder {
            get_info::Builder {
                client: self.0.clone(),
                metadata: metadata.into(),
            }
        }
    }
    pub mod get_token {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::IdentityErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
            pub(crate) resource: String,
            pub(crate) client_id: Option<String>,
            pub(crate) object_id: Option<String>,
            pub(crate) msi_res_id: Option<String>,
            pub(crate) authority: Option<String>,
            pub(crate) bypass_cache: Option<String>,
        }
        impl Builder {
            pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
                self.client_id = Some(client_id.into());
                self
            }
            pub fn object_id(mut self, object_id: impl Into<String>) -> Self {
                self.object_id = Some(object_id.into());
                self
            }
            pub fn msi_res_id(mut self, msi_res_id: impl Into<String>) -> Self {
                self.msi_res_id = Some(msi_res_id.into());
                self
            }
            pub fn authority(mut self, authority: impl Into<String>) -> Self {
                self.authority = Some(authority.into());
                self
            }
            pub fn bypass_cache(mut self, bypass_cache: impl Into<String>) -> Self {
                self.bypass_cache = Some(bypass_cache.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::IdentityTokenResponse, Error>> {
                Box::pin(async move {
                    let url_str = &format!("{}/identity/oauth2/token", self.client.endpoint(),);
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2019-03-11");
                    req_builder = req_builder.header("Metadata", &self.metadata);
                    let resource = &self.resource;
                    url.query_pairs_mut().append_pair("resource", resource);
                    if let Some(client_id) = &self.client_id {
                        url.query_pairs_mut().append_pair("client_id", client_id);
                    }
                    if let Some(object_id) = &self.object_id {
                        url.query_pairs_mut().append_pair("object_id", object_id);
                    }
                    if let Some(msi_res_id) = &self.msi_res_id {
                        url.query_pairs_mut().append_pair("msi_res_id", msi_res_id);
                    }
                    if let Some(authority) = &self.authority {
                        url.query_pairs_mut().append_pair("authority", authority);
                    }
                    if let Some(bypass_cache) = &self.bypass_cache {
                        url.query_pairs_mut().append_pair("bypass_cache", bypass_cache);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::IdentityTokenResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::IdentityErrorResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod get_info {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::IdentityErrorResponse,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) metadata: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::IdentityInfoResponse, Error>> {
                Box::pin(async move {
                    let url_str = &format!("{}/identity/info", self.client.endpoint(),);
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2019-03-11");
                    req_builder = req_builder.header("Metadata", &self.metadata);
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::IdentityInfoResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::IdentityErrorResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
