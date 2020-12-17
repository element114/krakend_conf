A tool to generate krakend endpoint configuration from openapi json, built on the openapiv3 crate.

oas2krakend generates a full krakend json config file.
The default template contains a rather permissive CORS config out of the box, feel free to change it according to your needs.

# Minimum rust version 1.35.0
 - cargo +1.35.0 test
 - cargo +1.35.0 build

# Looking for contributors

# Build and use the cli tool
```bash
cd oas2krakend
cargo build
...
oas2krakend -s openapi.json -c krakend_config.json
```
