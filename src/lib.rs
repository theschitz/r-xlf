use wasm_bindgen::prelude::*;
extern crate serde;
extern crate quick_xml;
extern crate serde_json;
extern crate serde_xml;
mod minimal_xlf;
mod xliff_structs;
use xliff_structs::Xliff;
use minimal_xlf::MinimalXliff;
use quick_xml::de::{DeError};

#[macro_use] extern crate serde_derive;

fn de_minimal_xlf(xml: String) -> Result<MinimalXliff, DeError> {
    let min_xlf: MinimalXliff = quick_xml::de::from_str(&xml)?;
    Ok(min_xlf)
}

fn get_xlf(xml: String) -> Result<Xliff, DeError> {
    let xlf: Xliff = quick_xml::de::from_str(&xml)?;
    Ok(xlf)
}

#[wasm_bindgen]
pub fn xlf_to_json(xml: String) -> String {
    let xlf: Xliff = get_xlf(xml).unwrap();
    return serde_json::to_string(&xlf).unwrap();
}

#[wasm_bindgen]
pub fn xlf_to_min_json(xml: String) -> String {
    let min_xlf: MinimalXliff = de_minimal_xlf(xml).unwrap();
    let json = serde_json::to_string(&min_xlf).unwrap();
    return json;
}

#[wasm_bindgen]
pub fn json_to_xlf(json: String) -> String {
    let deserialized: Xliff = serde_json::from_str(&json).unwrap();
    return quick_xml::se::to_string(&deserialized).unwrap()
}


fn sample_xlf() -> &'static str {
    return r#"<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="urn:oasis:names:tc:xliff:document:1.2 xliff-core-1.2-transitional.xsd"><file datatype="xml" source-language="en-US" target-language="sv-SE" original="Al"><body><group id="body"><trans-unit id="Table 596208023 - Property 2879900210" maxwidth="23" size-unit="char" translate="yes" xml:space="preserve"><source>State</source><note from="Developer" annotates="general" priority="2">TableComment</note><note from="Xliff Generator" annotates="general" priority="3">Table NAB Test Table - Property Caption</note></trans-unit><trans-unit id="Table 596208023 - Field 440443472 - Property 2879900210" size-unit="char" translate="yes" xml:space="preserve"><source>Field</source><target>asdf</target><note from="Developer" annotates="general" priority="2"></note><note from="Xliff Generator" annotates="general" priority="3">Table NAB Test Table - Field Test Field - Property Caption</note></trans-unit></group></body></file></xliff>"#;
    // return r#"<?xml version="1.0" encoding="utf-8"?><xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="urn:oasis:names:tc:xliff:document:1.2 xliff-core-1.2-transitional.xsd"><file datatype="xml" source-language="en-US" target-language="sv-SE" original="Al"><body><group id="body"><trans-unit id="Table 596208023 - Property 2879900210" maxwidth="23" size-unit="char" translate="yes" xml:space="preserve"><source>State</source><note from="Developer" annotates="general" priority="2">TableComment</note><note from="Xliff Generator" annotates="general" priority="3">Table NAB Test Table - Property Caption</note></trans-unit><trans-unit id="Table 596208023 - Field 440443472 - Property 2879900210" size-unit="char" translate="yes" xml:space="preserve"><source>Field</source><target>asdf</target><note from="Developer" annotates="general" priority="2"></note><note from="Xliff Generator" annotates="general" priority="3">Table NAB Test Table - Field Test Field - Property Caption</note></trans-unit></group></body></file></xliff>"#;
}

fn sample_json() -> &'static str {
    return r#"{"version":"1.2","xmlns":"urn:oasis:names:tc:xliff:document:1.2","xmlns:xsi":"http://www.w3.org/2001/XMLSchema-instance","xsi:schemaLocation":"urn:oasis:names:tc:xliff:document:1.2 xliff-core-1.2-transitional.xsd","file":{"datatype":"xml","source-language":"en-US","target-language":"sv-SE","original":"Al","body":{"group":[{"id":"body","trans-unit":[{"id":"Table 596208023 - Property 2879900210","maxwidth":"23","size-unit":"char","translate":"yes","xml:space":"preserve","source":{"$value":"State"},"target":null,"note":[{"from":"Developer","priority":"2","annotates":"general","$value":"TableComment"},{"from":"Xliff Generator","priority":"3","annotates":"general","$value":"Table NAB Test Table - Property Caption"}]},{"id":"Table 596208023 - Field 440443472 - Property 2879900210","maxwidth":null,"size-unit":"char","translate":"yes","xml:space":"preserve","source":{"$value":"Field"},"target":{"state":null,"$value":"asdf"},"note":[{"from":"Developer","priority":"2","annotates":"general","$value":null},{"from":"Xliff Generator","priority":"3","annotates":"general","$value":"Table NAB Test Table - Field Test Field - Property Caption"}]}]}]}}}"#
}

#[test]
fn test_xlf_to_json() {
    let json = xlf_to_json(sample_xlf().to_string());
    // println!("{}",json );
    assert_eq!(json,sample_json().to_string(), "Unexpected JSON.");
}

#[test]
fn test_json_to_xlf() {
    let xml = json_to_xlf(sample_json().to_string());
    assert_eq!(xml, sample_xlf(), "Unexpected XML.");
}
