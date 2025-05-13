// Creates a fuzzyfile for Methone

use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use super::WebDocument;

/// Generates a fuzzyfile for Methone based on the [WebDocument]s, so that each styrdokument can be
/// found by searching on the top bar (locally). This will place the `fuzzyfile` in the `public`
/// directory amond the pdf-files.
pub fn generate_fuzzyfile(web_documents: &HashMap<String, WebDocument>) {
    let path = "./public/fuzzyfile";
    let file = File::create(path).expect("Cannot create navbar template");
    let mut writer = BufWriter::new(file);

    writeln!(
        &mut writer,
        r#"{{
    "@type": "fuzzyfile",
    "fuzzes": ["#
    )
    .unwrap();

    for (i, (url, doc)) in web_documents.iter().enumerate() {
        writeln!(
            &mut writer,
            r#"        {{
            "name": "{}",
            "str": "{}",
            "href": "/dokument/{}"
        }}{}"#,
            doc.name(),
            doc.name().to_lowercase(),
            url,
            // because json is a cringe fucking shit language the last thing cannot have a ','
            if i + 1 < web_documents.len() { "," } else { "" }
        )
        .unwrap();
    }

    writeln!(
        &mut writer,
        r#"     ]
}}"#
    )
    .unwrap();
}
