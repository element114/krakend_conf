use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Swagger {
    /// The base path to the API. Example: '/api'.
    #[serde(rename = "basePath")]
    pub base_path: Option<String>,
    /// A list of MIME types accepted by the API.
    pub consumes: Option<Vec<String>>,
    /// One or more JSON objects describing the schemas being consumed and produced by the API.
    pub definitions: Option<HashMap<String, Schema>>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
    /// The fully qualified URI to the host of the API.
    pub host: Option<String>,
    pub info: Info,
    /// One or more JSON representations for parameters
    pub parameters: Option<HashMap<String, Parameter>>,
    /// Relative paths to the individual endpoints. They must be relative to the 'basePath'.
    pub paths: Paths,
    /// A list of MIME types the API can produce.
    pub produces: Option<Vec<String>>,
    pub responses: Option<Responses>,
    /// The transfer protocol of the API.
    pub schemes: Option<Vec<Scheme>>,
    pub security: Option<serde_json::Value>,
    /// The Swagger version of this document.
    pub swagger: f64,
    pub tags: Option<Vec<Tag>>,
}

/// A deterministic version of a JSON Schema object.
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub schema_ref: Option<String>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<Schema>>,
    #[serde(rename = "default")]
    pub schema_default: Option<serde_json::Value>,
    pub description: Option<String>,
    pub discriminator: Option<String>,
    #[serde(rename = "enum")]
    pub schema_enum: Option<Vec<Option<serde_json::Value>>>,
    pub example: Option<serde_json::Value>,
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
    pub format: Option<String>,
    pub items: Option<Items>,
    pub maximum: Option<f64>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<i64>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxProperties")]
    pub max_properties: Option<i64>,
    pub minimum: Option<f64>,
    #[serde(rename = "minItems")]
    pub min_items: Option<i64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minProperties")]
    pub min_properties: Option<i64>,
    #[serde(rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    pub pattern: Option<String>,
    pub properties: Option<HashMap<String, Schema>>,
    pub required: Option<Vec<String>>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub schema_type: Option<TypeUnion>,
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    pub xml: Option<Xml>,
}

/// information about external documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDocs {
    pub description: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XmlClass {
    pub attribute: Option<bool>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub wrapped: Option<bool>,
}

/// General information about the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    /// Contact information for the owners of the API.
    pub contact: Option<Contact>,
    /// A longer description of the API. Should be different from the title.  Github-flavored
    /// markdown is allowed.
    pub description: Option<String>,
    pub license: Option<License>,
    /// The terms of service for the API.
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    /// A unique and precise title of the API.
    pub title: String,
    /// A semantic version number of the API.
    pub version: String,
}

/// Contact information for the owners of the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    /// The email address of the contact person/organization.
    pub email: Option<String>,
    /// The identifying name of the contact person/organization.
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    /// The name of the license type. It's encouraged to use an OSI compatible license.
    pub name: String,
    /// The URL pointing to the license.
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// Determines the location of the parameter.
    #[serde(rename = "in")]
    pub parameter_in: In,
    #[serde(rename = "collectionFormat")]
    pub collection_format: Option<String>,
    /// A brief description of the parameter. This could contain examples of use.
    /// Github-flavored markdown is allowed.
    ///
    /// A brief description of the parameter. This could contain examples of use.
    pub description: Option<String>,
    pub format: Option<String>,
    pub items: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Determines whether or not this parameter is required or optional.
    pub required: Option<bool>,
    #[serde(rename = "type")]
    pub parameter_type: Option<TypeEnum>,
    pub schema: Option<Box<Schema>>,
}

/// Relative paths to the individual endpoints. They must be relative to the 'basePath'.
#[derive(Debug, Serialize, Deserialize)]
pub struct Paths {
}

/// Response objects names can either be any valid HTTP status code or 'default'.
#[derive(Debug, Serialize, Deserialize)]
pub struct Responses {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Schema(Box<Schema>),
    SchemaArray(Vec<Schema>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeUnion {
    Enum(SimpleTypes),
    EnumArray(Vec<SimpleTypes>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Xml {
    AnythingArray(Vec<Option<serde_json::Value>>),
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    XmlClass(XmlClass),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SimpleTypes {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}

/// Determines the location of the parameter.
#[derive(Debug, Serialize, Deserialize)]
pub enum In {
    #[serde(rename = "body")]
    Body,
    #[serde(rename = "formData")]
    FormData,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "query")]
    Query,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Scheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "ws")]
    Ws,
    #[serde(rename = "wss")]
    Wss,
}
