#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
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
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{endpoint}/")]);
        Client::new(endpoint, self.credential, scopes, self.options)
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
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
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
    pub fn deployments_client(&self) -> deployments::Client {
        deployments::Client(self.clone())
    }
    pub fn devices_client(&self) -> devices::Client {
        devices::Client(self.clone())
    }
    pub fn updates_client(&self) -> updates::Client {
        updates::Client(self.clone())
    }
}
pub mod deployments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Cancels a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn cancel_deployment(
            &self,
            instance_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> cancel_deployment::RequestBuilder {
            cancel_deployment::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Retries a deployment with failed devices."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn retry_deployment(
            &self,
            instance_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> retry_deployment::RequestBuilder {
            retry_deployment::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Gets a list of deployments."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_all_deployments(&self, instance_id: impl Into<String>) -> get_all_deployments::RequestBuilder {
            get_all_deployments::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: None,
            }
        }
        #[doc = "Gets the properties of a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_deployment(&self, instance_id: impl Into<String>, deployment_id: impl Into<String>) -> get_deployment::RequestBuilder {
            get_deployment::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Creates or updates a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        #[doc = "* `deployment`: The deployment properties."]
        pub fn create_or_update_deployment(
            &self,
            instance_id: impl Into<String>,
            deployment_id: impl Into<String>,
            deployment: impl Into<models::Deployment>,
        ) -> create_or_update_deployment::RequestBuilder {
            create_or_update_deployment::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
                deployment: deployment.into(),
            }
        }
        #[doc = "Deletes a deployment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn delete_deployment(
            &self,
            instance_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> delete_deployment::RequestBuilder {
            delete_deployment::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Gets the status of a deployment including a breakdown of how many devices in the deployment are in progress, completed, or failed."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_deployment_status(
            &self,
            instance_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> get_deployment_status::RequestBuilder {
            get_deployment_status::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
            }
        }
        #[doc = "Gets a list of devices in a deployment along with their state. Useful for getting a list of failed devices."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `deployment_id`: Deployment identifier."]
        pub fn get_deployment_devices(
            &self,
            instance_id: impl Into<String>,
            deployment_id: impl Into<String>,
        ) -> get_deployment_devices::RequestBuilder {
            get_deployment_devices::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                deployment_id: deployment_id.into(),
                filter: None,
            }
        }
    }
    pub mod cancel_deployment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Deployment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Deployment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}?action=cancel",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Deployment>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Deployment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod retry_deployment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Deployment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Deployment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}?action=retry",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Deployment>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Deployment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_deployments {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfDeployments> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfDeployments = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Restricts the set of deployments returned. You can filter on update Provider, Name and Version property."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfDeployments, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_deployment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Deployment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Deployment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Deployment>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Deployment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update_deployment {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Deployment> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Deployment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
            pub(crate) deployment: models::Deployment,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.deployment)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Deployment>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Deployment>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_deployment {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_deployment_status {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeploymentStatus> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DeploymentStatus = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}/status",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeploymentStatus>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::DeploymentStatus>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_deployment_devices {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfDeploymentDeviceStates> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfDeploymentDeviceStates = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) deployment_id: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Restricts the set of deployment device states returned. You can filter on deviceId and/or deviceState."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfDeploymentDeviceStates, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deployments/{}/devicestates",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.deployment_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod updates {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Import new update version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `action`: Import update action."]
        #[doc = "* `update_to_import`: The update to be imported."]
        pub fn import_update(
            &self,
            instance_id: impl Into<String>,
            action: impl Into<String>,
            update_to_import: impl Into<models::ImportUpdateInput>,
        ) -> import_update::RequestBuilder {
            import_update::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                action: action.into(),
                update_to_import: update_to_import.into(),
            }
        }
        #[doc = "Get a specific update version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        pub fn get_update(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
        ) -> get_update::RequestBuilder {
            get_update::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
                if_none_match: None,
            }
        }
        #[doc = "Delete a specific update version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        pub fn delete_update(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
        ) -> delete_update::RequestBuilder {
            delete_update::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
            }
        }
        #[doc = "Get a list of all update providers that have been imported to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_providers(&self, instance_id: impl Into<String>) -> get_providers::RequestBuilder {
            get_providers::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Get a list of all update names that match the specified provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        pub fn get_names(&self, instance_id: impl Into<String>, provider: impl Into<String>) -> get_names::RequestBuilder {
            get_names::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
            }
        }
        #[doc = "Get a list of all update versions that match the specified provider and name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        pub fn get_versions(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
        ) -> get_versions::RequestBuilder {
            get_versions::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
            }
        }
        #[doc = "Get a list of all update file identifiers for the specified version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        pub fn get_files(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
        ) -> get_files::RequestBuilder {
            get_files::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
            }
        }
        #[doc = "Get a specific update file from the version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `provider`: Update provider."]
        #[doc = "* `name`: Update name."]
        #[doc = "* `version`: Update version."]
        #[doc = "* `file_id`: File identifier."]
        pub fn get_file(
            &self,
            instance_id: impl Into<String>,
            provider: impl Into<String>,
            name: impl Into<String>,
            version: impl Into<String>,
            file_id: impl Into<String>,
        ) -> get_file::RequestBuilder {
            get_file::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                provider: provider.into(),
                name: name.into(),
                version: version.into(),
                file_id: file_id.into(),
                if_none_match: None,
            }
        }
        #[doc = "Get a list of all import update operations. Completed operations are kept for 7 days before auto-deleted. Delete operations are not returned by this API version."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_operations(&self, instance_id: impl Into<String>) -> get_operations::RequestBuilder {
            get_operations::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: None,
                top: None,
            }
        }
        #[doc = "Retrieve operation status."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `operation_id`: Operation identifier."]
        pub fn get_operation(&self, instance_id: impl Into<String>, operation_id: impl Into<String>) -> get_operation::RequestBuilder {
            get_operation::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                operation_id: operation_id.into(),
                if_none_match: None,
            }
        }
    }
    pub mod import_update {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) action: String,
            pub(crate) update_to_import: models::ImportUpdateInput,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/deviceupdate/{}/v2/updates", this.client.endpoint(), &this.instance_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let action = &this.action;
                        req.url_mut().query_pairs_mut().append_pair("action", action);
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_to_import)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_update {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Update> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Update = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers/{}/names/{}/versions/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Update>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Update>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_update {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers/{}/names/{}/versions/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_providers {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfStrings> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfStrings = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfStrings, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_names {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfStrings> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfStrings = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfStrings, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers/{}/names",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_versions {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfStrings> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfStrings = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfStrings, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers/{}/names/{}/versions",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_files {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfStrings> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfStrings = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfStrings, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers/{}/names/{}/versions/{}/files",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_file {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::File> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::File = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) provider: String,
            pub(crate) name: String,
            pub(crate) version: String,
            pub(crate) file_id: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/providers/{}/names/{}/versions/{}/files/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.provider,
                            &this.name,
                            &this.version,
                            &this.file_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::File>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::File>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_operations {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfOperations> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfOperations = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "Restricts the set of operations returned. Only one specific filter is supported: \"status eq 'NotStarted' or status eq 'Running'\""]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Specifies a non-negative integer n that limits the number of items returned from a collection. The service returns the number of available items up to but not greater than the specified value n."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfOperations, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/operations",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_operation {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Operation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Operation = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "Number of seconds to wait before checking the operation status again."]
            pub fn retry_after(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) operation_id: String,
            pub(crate) if_none_match: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Defines the If-None-Match condition. The operation will be performed only if the ETag on the server does not match this value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/updates/operations/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.operation_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Operation>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Operation>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod devices {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of all device classes (unique combinations of device manufacturer and model) for all devices connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_all_device_classes(&self, instance_id: impl Into<String>) -> get_all_device_classes::RequestBuilder {
            get_all_device_classes::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Gets the properties of a device class."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_device_class(
            &self,
            instance_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> get_device_class::RequestBuilder {
            get_device_class::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Gets a list of device identifiers in a device class."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_device_class_device_ids(
            &self,
            instance_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> get_device_class_device_ids::RequestBuilder {
            get_device_class_device_ids::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Gets a list of installable updates for a device class."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_class_id`: Device class identifier."]
        pub fn get_device_class_installable_updates(
            &self,
            instance_id: impl Into<String>,
            device_class_id: impl Into<String>,
        ) -> get_device_class_installable_updates::RequestBuilder {
            get_device_class_installable_updates::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_class_id: device_class_id.into(),
            }
        }
        #[doc = "Gets a list of devices connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_all_devices(&self, instance_id: impl Into<String>) -> get_all_devices::RequestBuilder {
            get_all_devices::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                filter: None,
            }
        }
        #[doc = "Gets the device properties and latest deployment status for a device connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `device_id`: Device identifier in Azure IOT Hub."]
        pub fn get_device(&self, instance_id: impl Into<String>, device_id: impl Into<String>) -> get_device::RequestBuilder {
            get_device::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                device_id: device_id.into(),
            }
        }
        #[doc = "Gets the breakdown of how many devices are on their latest update, have new updates available, or are in progress receiving new updates."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_update_compliance(&self, instance_id: impl Into<String>) -> get_update_compliance::RequestBuilder {
            get_update_compliance::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Gets a list of available group device tags for all devices connected to Device Update for IoT Hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_all_device_tags(&self, instance_id: impl Into<String>) -> get_all_device_tags::RequestBuilder {
            get_all_device_tags::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Gets a count of how many devices have a device tag."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `tag_name`: Tag name."]
        pub fn get_device_tag(&self, instance_id: impl Into<String>, tag_name: impl Into<String>) -> get_device_tag::RequestBuilder {
            get_device_tag::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                tag_name: tag_name.into(),
            }
        }
        #[doc = "Gets a list of all device groups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        pub fn get_all_groups(&self, instance_id: impl Into<String>) -> get_all_groups::RequestBuilder {
            get_all_groups::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
            }
        }
        #[doc = "Gets the properties of a group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identifier."]
        pub fn get_group(&self, instance_id: impl Into<String>, group_id: impl Into<String>) -> get_group::RequestBuilder {
            get_group::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Create or update a device group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identifier."]
        #[doc = "* `group`: The group properties."]
        pub fn create_or_update_group(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
            group: impl Into<models::Group>,
        ) -> create_or_update_group::RequestBuilder {
            create_or_update_group::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                group: group.into(),
            }
        }
        #[doc = "Deletes a device group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identifier."]
        pub fn delete_group(&self, instance_id: impl Into<String>, group_id: impl Into<String>) -> delete_group::RequestBuilder {
            delete_group::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Get group update compliance information such as how many devices are on their latest update, how many need new updates, and how many are in progress on receiving a new update."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identifier."]
        pub fn get_group_update_compliance(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> get_group_update_compliance::RequestBuilder {
            get_group_update_compliance::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
            }
        }
        #[doc = "Get the best available updates for a group and a count of how many devices need each update."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `instance_id`: Account instance identifier."]
        #[doc = "* `group_id`: Group identifier."]
        pub fn get_group_best_updates(
            &self,
            instance_id: impl Into<String>,
            group_id: impl Into<String>,
        ) -> get_group_best_updates::RequestBuilder {
            get_group_best_updates::RequestBuilder {
                client: self.0.clone(),
                instance_id: instance_id.into(),
                group_id: group_id.into(),
                filter: None,
            }
        }
    }
    pub mod get_all_device_classes {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfDeviceClasses> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfDeviceClasses = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfDeviceClasses, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deviceclasses",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_device_class {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeviceClass> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DeviceClass = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deviceclasses/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeviceClass>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::DeviceClass>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_device_class_device_ids {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfStrings> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfStrings = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfStrings, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deviceclasses/{}/deviceids",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_device_class_installable_updates {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfUpdateIds> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfUpdateIds = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_class_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfUpdateIds, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/deviceclasses/{}/installableupdates",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_class_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_all_devices {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfDevices> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfDevices = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Restricts the set of devices returned. You can only filter on device GroupId."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfDevices, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/devices",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_device {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Device> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Device = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) device_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/devices/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.device_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Device>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Device>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_update_compliance {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::UpdateCompliance> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::UpdateCompliance = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/updatecompliance",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::UpdateCompliance>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::UpdateCompliance>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_device_tags {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfDeviceTags> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfDeviceTags = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfDeviceTags, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/devicetags",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_device_tag {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DeviceTag> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DeviceTag = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) tag_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/devicetags/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.tag_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DeviceTag>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::DeviceTag>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_all_groups {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfGroups> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfGroups = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfGroups, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/groups",
                            this.client.endpoint(),
                            &this.instance_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Group> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Group = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/groups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Group>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Group>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod create_or_update_group {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Group> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Group = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) group: models::Group,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/groups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.group)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Group>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::Group>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod delete_group {
        use super::models;
        pub struct Response(azure_core::Response);
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/groups/{}",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_group_update_compliance {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::UpdateCompliance> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::UpdateCompliance = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/groups/{}/updateCompliance",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::UpdateCompliance>;
            type IntoFuture = futures::future::BoxFuture<'static, azure_core::Result<models::UpdateCompliance>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
    pub mod get_group_best_updates {
        use super::models;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PageableListOfUpdatableDevices> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PageableListOfUpdatableDevices = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) instance_id: String,
            pub(crate) group_id: String,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Restricts the set of bestUpdates returned. You can filter on update Provider, Name and Version property."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::PageableListOfUpdatableDevices, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/deviceupdate/{}/v2/management/groups/{}/bestUpdates",
                            this.client.endpoint(),
                            &this.instance_id,
                            &this.group_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
