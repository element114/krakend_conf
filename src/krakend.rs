use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Krakend {
    pub version: i64,
    pub extra_config: ExtraConfig,
    pub timeout: String,
    pub cache_ttl: String,
    pub output_encoding: String,
    pub name: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Endpoint {
    pub endpoint: String,
    pub method: String,
    pub extra_config: ExtraConfig,
    #[serde(
        skip_serializing_if = "is_default_output_encoding",
        default = "default_output_encoding"
    )]
    pub output_encoding: String,
    #[serde(
        skip_serializing_if = "is_default_concurrent_calls",
        default = "default_concurrent_calls"
    )]
    pub concurrent_calls: i64,
    pub backend: Vec<Backend>,
    pub headers_to_pass: Vec<String>,
}
fn default_output_encoding() -> String {
    "json".to_string()
}
fn is_default_output_encoding(inp: &str) -> bool {
    default_output_encoding().eq(inp)
}
fn default_concurrent_calls() -> i64 {
    1
}
fn is_default_concurrent_calls(inp: &i64) -> bool {
    default_concurrent_calls().eq(inp)
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Backend {
    pub url_pattern: String,
    #[serde(
        skip_serializing_if = "is_default_encoding",
        default = "default_encoding"
    )]
    pub encoding: String,
    #[serde(skip_serializing_if = "is_default_sd", default = "default_sd")]
    pub sd: String,
    #[serde(skip_serializing_if = "is_default_extra_config", default)]
    pub extra_config: ExtraConfig,
    pub host: Vec<String>,
    #[serde(
        skip_serializing_if = "is_default_disable_host_sanitize",
        default = "default_disable_host_sanitize"
    )]
    pub disable_host_sanitize: bool,
    pub is_collection: bool,
}
fn default_disable_host_sanitize() -> bool {
    false
}
fn is_default_disable_host_sanitize(inp: &bool) -> bool {
    default_disable_host_sanitize().eq(inp)
}
fn default_sd() -> String {
    "static".to_string()
}
fn is_default_sd(inp: &str) -> bool {
    default_sd().eq(inp)
}
fn default_encoding() -> String {
    "json".to_string()
}
fn is_default_encoding(inp: &str) -> bool {
    default_encoding().eq(inp)
}
fn is_default_extra_config(inp: &ExtraConfig) -> bool {
    ExtraConfig::default().eq(inp)
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ExtraConfig;
