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
    state: Option<String>,
    #[serde(rename = "$value", default)]
    value: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Note {
    from: Option<String>,
    #[serde(rename = "$value", default)]
    value: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TransUnit {
    id: Option<String>,
    source: Source,
    target: Option<Target>,
    #[serde(rename = "note", default)]
    notes: Vec<Note>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Group {
    id: Option<String>,
    #[serde(rename = "trans-unit", default)]
    trans_units: Vec<TransUnit> 
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Body {
    group: Vec<Group>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct File {
    datatype: String,
    #[serde(rename = "source-language", default)]
    source_language: String,
    #[serde(rename = "target-language", default)]
    target_language: Option<String>,
    original: String,
    body: Body
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Xliff {
    version: String,
    xmlns: String,
    #[serde(rename = "xmlns:xsi", default)]
    xmlns_xsi: String,
    file: File
}