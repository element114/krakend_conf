mod krakend;

use krakend::{Backend, Endpoint, ExtraConfig};
use openapiv3::{OpenAPI, PathItem};

fn main() {
    std::env::set_var("RUST_BACKTRACE","1");
    let data = include_str!("../openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");

    let srv = openapi.servers.first().unwrap();
    let base_path = srv.url.clone();

    let mut endpoints = Vec::new();
    for path in openapi.paths {
        let pit = serde_yaml::to_string(&path.1).unwrap();
        let dpath: PathItem = serde_yaml::from_str(&pit).unwrap();
        if let Some(op) = dpath.get {
            // println!("{:?}", op.tags);
            let mut e = Endpoint::default();
            e.endpoint = path.0.clone();
            e.method = "GET".to_owned();
            e.output_encoding = "json".to_owned();
            e.concurrent_calls = 1i64;
            let b = Backend {
                url_pattern: format!("{}{}", base_path, path.0.clone()),
                encoding: "json".to_owned(),
                sd: "static".to_owned(),
                extra_config: ExtraConfig::default(),
                host: vec!["http://127.0.0.1".to_owned()],
                disable_host_sanitize: false,
            };
            e.backend = vec![b];
            endpoints.push(e);
        }
        if let Some(op) = dpath.post {
            let mut e = Endpoint::default();
            e.endpoint = path.0.clone();
            e.method = "POST".to_owned();
            e.output_encoding = "json".to_owned();
            e.concurrent_calls = 1i64;
            let b = Backend {
                url_pattern: format!("{}{}", base_path, path.0.clone()),
                encoding: "json".to_owned(),
                sd: "static".to_owned(),
                extra_config: ExtraConfig::default(),
                host: vec!["http://127.0.0.1".to_owned()],
                disable_host_sanitize: false,
            };
            e.backend = vec![b];
            endpoints.push(e);
        }
        if let Some(op) = dpath.put {
            let mut e = Endpoint::default();
            e.endpoint = path.0.clone();
            e.method = "PUT".to_owned();
            e.output_encoding = "json".to_owned();
            e.concurrent_calls = 1i64;
            let b = Backend {
                url_pattern: format!("{}{}", base_path, path.0.clone()),
                encoding: "json".to_owned(),
                sd: "static".to_owned(),
                extra_config: ExtraConfig::default(),
                host: vec!["http://127.0.0.1".to_owned()],
                disable_host_sanitize: false,
            };
            e.backend = vec![b];
            endpoints.push(e);
        }
        if let Some(op) = dpath.patch {
            let mut e = Endpoint::default();
            e.endpoint = path.0.clone();
            e.method = "PATCH".to_owned();
            e.output_encoding = "json".to_owned();
            e.concurrent_calls = 1i64;
            let b = Backend {
                url_pattern: format!("{}{}", base_path, path.0.clone()),
                encoding: "json".to_owned(),
                sd: "static".to_owned(),
                extra_config: ExtraConfig::default(),
                host: vec!["http://127.0.0.1".to_owned()],
                disable_host_sanitize: false,
            };
            e.backend = vec![b];
            endpoints.push(e);
        }
        if let Some(op) = dpath.delete {
            let mut e = Endpoint::default();
            e.endpoint = path.0.clone();
            e.method = "DELETE".to_owned();
            e.output_encoding = "json".to_owned();
            e.concurrent_calls = 1i64;
            let b = Backend {
                url_pattern: format!("{}{}", base_path, path.0.clone()),
                encoding: "json".to_owned(),
                sd: "static".to_owned(),
                extra_config: ExtraConfig::default(),
                host: vec!["http://127.0.0.1".to_owned()],
                disable_host_sanitize: false,
            };
            e.backend = vec![b];
            endpoints.push(e);
        }
    }

    let eps = serde_json::to_string(&endpoints);
    let res = std::fs::write("krakend_endpoints.json", eps.unwrap().as_bytes());
    println!("{:#?}", res);
}
