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

// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     // alert(&format!("Hello, {}!", name));
//     alert(&format!("{}", xlf_to_min_json(r#"<?xml version="1.0" encoding="utf-8"?>
//     <xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="urn:oasis:names:tc:xliff:document:1.2 xliff-core-1.2-transitional.xsd">
//       <file datatype="xml" source-language="en-US" target-language="sv-SE" original="AlTestApp">
//         <body>
//           <group id="body">
//             <trans-unit id="Table 2328808854 - NamedType 12557645" size-unit="char" maxwidth="50" translate="yes" xml:space="preserve">
//               <source>This is a test</source>
//               <target state="final" state-qualifier="exact-match">Detta är ett test</target>
//               <note from="Developer" annotates="general" priority="2">Some kind of Dev note</note>
//               <note from="Xliff Generator" annotates="general" priority="3">Table MyTable - NamedType TestErr</note>
//               <note from="NAB AL Tool Refresh Xlf" annotates="general" priority="3">Source has been modified.</note>
//             </trans-unit>
//             <trans-unit id="Page 2931038265 - NamedType 12557645" size-unit="char" translate="yes" xml:space="preserve">
//               <source>Cool</source>
//               <target>Sval</target>
//               <note from="Developer" annotates="general" priority="2"/>
//               <note from="Xliff Generator" annotates="general" priority="3">Page MyPage - NamedType TestErr</note>
//             </trans-unit>
//           </group>
//         </body>
//       </file>
//     </xliff>"#.to_string())));
// }
