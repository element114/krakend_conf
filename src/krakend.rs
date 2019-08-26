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
    pub output_encoding: String,
    pub concurrent_calls: i64,
    pub backend: Vec<Backend>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Backend {
    pub url_pattern: String,
    pub encoding: String,
    pub sd: String,
    pub extra_config: ExtraConfig,
    pub host: Vec<String>,
    pub disable_host_sanitize: bool,
    pub is_collection: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExtraConfig;
