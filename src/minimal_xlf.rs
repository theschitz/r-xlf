extern crate serde;
extern crate quick_xml;
extern crate serde_json;
extern crate serde_xml;

use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Source {
    #[serde(rename = "$value", default)]
    value: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Target {
    #[serde(rename = "$value", default)]
    value: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TransUnit {
    source: Source,
    target: Option<Target>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Group {
    #[serde(rename = "trans-unit", default)]
    trans_units: Vec<TransUnit> 
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Body {
    group: Vec<Group>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct File {
    #[serde(rename = "source-language", default)]
    source_language: String,
    #[serde(rename = "target-language", default)]
    target_language: Option<String>,
    original: String,
    body: Body
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MinimalXliff {
    file: File
}