use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Contact {
    pub email: String,

    pub name: Option<String>,

    pub url: Option<String>,

    pub x_role: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct License {
    pub name: String,

    pub url: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ExternalDocs {
    pub description: String,

    pub url: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct XTranslator {
    pub infores: String,

    pub team: Vec<String>,

    pub component: String,

    #[serde(rename = "biolink-version")]
    pub biolink_version: Option<String>,

    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct XTrapi {
    pub async_query: Option<bool>,

    pub batch_size_limit: Option<i32>,

    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,

    pub operations: Vec<String>,

    pub rate_limit: Option<i32>,

    pub test_data_location: Option<Value>,

    pub version: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Info {
    pub contact: Contact,

    pub description: Option<String>,

    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,

    pub title: String,

    pub version: String,

    #[serde(rename = "x-translator")]
    pub x_translator: Option<XTranslator>,

    #[serde(rename = "x-trapi")]
    pub x_trapi: XTrapi,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Server {
    pub url: String,

    #[serde(rename = "x-maturity")]
    pub x_maturity: String,

    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Tag {
    pub name: String,

    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Hit {
    pub components: HashMap<String, Value>,

    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,

    pub info: Info,

    pub openapi: Option<String>,

    pub paths: HashMap<String, Value>,

    pub servers: Vec<Server>,

    pub tags: Vec<Tag>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct SmartAPIResponse {
    pub took: i32,

    pub total: i32,

    pub max_score: f32,

    pub hits: Vec<Hit>,
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::SmartAPIResponse;

    #[test]
    // #[ignore]
    fn scratch() {
        let data = fs::read_to_string("smartapi.pretty.json").unwrap();
        let potential_response: serde_json::Result<SmartAPIResponse> = serde_json::from_str(data.as_str());
        match potential_response {
            Err(error) => {
                println!("{}", error);
                assert!(false);
            }
            Ok(query) => {
                // let pretty_query = serde_json::to_string_pretty(&query).unwrap();
                // fs::write("/tmp/scratch-icees.pretty.json", pretty_query).unwrap();
                assert!(true);
            }
        }
    }
}
