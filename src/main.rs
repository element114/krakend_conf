mod krakend;

use krakend::{Backend, Endpoint, ExtraConfig};
use openapiv3::{OpenAPI, PathItem};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let data = include_str!("../openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");

    let srv = openapi.servers.first().unwrap();
    let base_path = srv.url.clone();
    let title = openapi.info.title;

    let mut endpoints = Vec::new();
    for path in openapi.paths {
        let pit = serde_yaml::to_string(&path.1).unwrap();
        let dpath: PathItem = serde_yaml::from_str(&pit).unwrap();
        let pth = format!("/{}{}", title, path.0);
        let backend_pth = format!("{}{}", base_path, path.0.clone());
        let hosts = vec!["http://127.0.0.1:8529".to_owned()];
        if let Some(_op) = dpath.get {
            let method = "GET".to_owned();
            let e = create_endpoint(pth.clone(), backend_pth.clone(), hosts.clone(), method);
            endpoints.push(e);
        }
        if let Some(_op) = dpath.post {
            let method = "POST".to_owned();
            let e = create_endpoint(pth.clone(), backend_pth.clone(), hosts.clone(), method);
            endpoints.push(e);
        }
        if let Some(_op) = dpath.put {
            let method = "PUT".to_owned();
            let e = create_endpoint(pth.clone(), backend_pth.clone(), hosts.clone(), method);
            endpoints.push(e);
        }
        if let Some(_op) = dpath.patch {
            let method = "PATCH".to_owned();
            let e = create_endpoint(pth.clone(), backend_pth.clone(), hosts.clone(), method);
            endpoints.push(e);
        }
        if let Some(_op) = dpath.delete {
            let method = "DELETE".to_owned();
            let e = create_endpoint(pth.clone(), backend_pth.clone(), hosts.clone(), method);
            endpoints.push(e);
        }
    }

    let eps = serde_json::to_string(&endpoints);
    let res = std::fs::write("krakend_endpoints.json", eps.unwrap().as_bytes());
    println!("{:#?}", res);
}

fn create_endpoint(
    pth: String,
    backend_pth: String,
    hosts: Vec<String>,
    method: String,
) -> Endpoint {
    let is_collection;
    let mut enc = "no-op";
    if !pth.contains('{') && method.eq("GET") {
        // probably a collection
        is_collection = true;
        enc = "json";
    } else {
        is_collection = false;
    };
    let mut e = Endpoint::default();
    e.endpoint = pth;
    e.method = method;
    e.output_encoding = enc.to_owned();
    e.concurrent_calls = 1i64;
    let b = Backend {
        url_pattern: backend_pth,
        encoding: enc.to_owned(),
        sd: "static".to_owned(),
        extra_config: ExtraConfig::default(),
        host: hosts,
        disable_host_sanitize: false,
        is_collection
    };
    e.backend = vec![b];
    e
}
