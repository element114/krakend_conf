use clap::{App, Arg};
use openapiv3::OpenAPI;
use askama::Template;
// use krakend_conf::{Endpoint};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Andras Mocsary <425404+mocsy@users.noreply.github.com>")
        .about("Creates a krakend-ce configuration yaml from an OpenAPI 3 specification.")
        .arg(
            Arg::with_name("spec")
                .short("s")
                .long("spec")
                .takes_value(true)
                .help("Path to the OpenAPI specification JSON or YAML. default: openapi.json"),
        )
        .arg(
            Arg::with_name("conf")
                .short("c")
                .long("conf")
                .takes_value(true)
                .help("Path to the Krakend configuration Json output. default: krakend.json"),
        )
        .arg(
            Arg::with_name("hosts")
                .short("h")
                .long("hosts")
                .takes_value(true)
                .help("Comma separated list of host urls. default: http://127.0.0.1:8080"),
        )
        .get_matches();

    let oas_path = matches.value_of("spec").unwrap_or("openapi.json");
    let conf_path = matches.value_of("conf").unwrap_or("krakend.json");
    let hosts = if let Some(value) = matches.value_of("conf") {
        value.split(',').map(|s| s.to_string()).collect()
    } else {
        vec!["http://127.0.0.1:8080".to_owned()]
    };

    let openapi_str: String = std::fs::read_to_string(oas_path)
        .unwrap_or_else(|_| panic!("Expected a valid text file at: {}.", oas_path))
        .parse()
        .unwrap_or_else(|_| panic!("Expected a valid text file at: {}.", oas_path));

    let openapi_json = serde_json::from_str(&openapi_str);
    let openapi_yaml = serde_yaml::from_str(&openapi_str);

    let openapi_spec: OpenAPI = if let Ok(value) = openapi_json {
        value
    } else if let Ok(value) = openapi_yaml {
        value
    } else {
        panic!("Expected a valid OpenAPI spec in either yaml or json format.");
    };

    let endpoints = krakend_conf::convert_endpoints(openapi_spec, hosts);
    let eps = serde_json::to_string(&endpoints).unwrap();
   
    let conf_tmpl = KrakendTemplate { endpoints: eps.as_str() };
    let template = conf_tmpl.render().unwrap();
    let res = std::fs::write(conf_path, template.as_bytes());

    println!("{:#?}", res);
}

#[derive(Template)]
#[template(path = "krakend.json", escape = "none")]
struct KrakendTemplate<'a> {
    endpoints: &'a str,
}
