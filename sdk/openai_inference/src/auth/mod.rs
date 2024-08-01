use std::{borrow::Cow, convert::From};

use azure_core::{auth::Secret, headers::{AUTHORIZATION, HeaderName, HeaderValue}, Header};

pub struct AzureKeyCredential(Secret);

pub struct OpenAIKeyCredential (Secret);

impl OpenAIKeyCredential {
    pub fn new(access_token: String) -> Self {
        Self(Secret::new(access_token))
    }
}

impl Header for AzureKeyCredential {
    fn name(&self) -> HeaderName {
        AUTHORIZATION
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_cow(format!("api-key {}", self.0.secret()))
    }
}

impl Header for OpenAIKeyCredential {
    fn name(&self) -> HeaderName {
        AUTHORIZATION
    }

    fn value(&self) -> HeaderValue {
        HeaderValue::from_cow(format!("Bearer {}", &self.0.secret()))
    }
    
}
