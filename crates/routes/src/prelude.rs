#![allow(dead_code)]

pub use aide::{axum::ApiRouter, openapi::Tag};
pub use axum::http::StatusCode;
pub use schemars::JsonSchema;
use schemars::json_schema;
use serde::ser::SerializeStruct;
pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;

pub trait RouterExt {
    fn with_prefix(self, prefix: &'static str) -> Self;
    fn with_tag(self, tag_name: &'static str) -> Self;
}
impl RouterExt for ApiRouter {
    fn with_prefix(self, prefix: &'static str) -> Self {
        ApiRouter::new().nest_api_service(prefix, self)
    }
    fn with_tag(self, tag_name: &'static str) -> Self {
        self.with_path_items(|op| op.tag(tag_name))
    }
}

#[derive(Serialize, JsonSchema, Default, Clone)]
pub struct ApiResponse<T>
where
    T: JsonSchema,
{
    pub code: ApiStatusCode,
    pub resp: String,
    pub data: T,
}
impl<T: JsonSchema> ApiResponse<T> {
    pub fn code(mut self, code: ApiStatusCode) -> Self {
        self.code = code;
        self
    }
    pub fn resp(mut self, resp: String) -> Self {
        self.resp = resp;
        self
    }
    pub fn data(mut self, data: T) -> Self {
        self.data = data;
        self
    }
}

// Newtype for StatusCode
#[derive(Clone)]
pub struct ApiStatusCode(pub StatusCode);
impl JsonSchema for ApiStatusCode {
    fn inline_schema() -> bool {
        true
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
          "type": "integer",
          "format": "uint16",
          "maximum": 65535,
          "minimum": 0,
          "default": 200
        })
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::ApiStatusCode").into()
    }
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ApiStatusCode".into()
    }
}
impl Default for ApiStatusCode {
    fn default() -> Self {
        Self(StatusCode::NOT_IMPLEMENTED)
    }
}
impl Serialize for ApiStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0.as_u16())
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Clone)]
/// # Empty
/// Describes `null` state for compiler to understand.
/// ## How to use
/// ```
/// let response = ApiResponse<Empty> {
///     code: 0,
///     resp: "ok".to_string(),
///     data: Empty
/// };
/// ```
pub struct Empty;
