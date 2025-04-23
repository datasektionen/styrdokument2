use std::{collections::HashMap, fs};

use typst::text::{Font, FontBook};
use typst_pdf::PdfOptions;

use super::{
    typst_wrapper::{create_fontbook, Asgård},
    TypstDocument,
};

pub const HTML_DIRECTORY: &str = "documents/";
pub const PDF_DIRECTORY: &str = "public";
const TEMPLATE_PREPEND: &str = r#"{% extends "index" %}{% block content %}"#;
const TEMPLATE_APPEND: &str = r#"{% endblock content %}"#;

#[derive(Clone)]
pub struct WebDocument {
    name: String,
    filename: String,
    pdf_url: String,
}

impl WebDocument {
    fn new(value: &TypstDocument, pdf_url: String) -> Self {
        WebDocument {
            name: value.name().to_string(),
            filename: value.filename_name().to_string(),
            pdf_url,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn pdf_url(&self) -> &str {
        &self.pdf_url
    }
}

pub fn export(documents: &Vec<TypstDocument>) -> HashMap<String, WebDocument> {
    let (book, fonts) = create_fontbook();
    let mut document_mapping = HashMap::new();

    export_documents(&mut document_mapping, documents, book, fonts, None);

    document_mapping
}

fn export_documents(
    map: &mut HashMap<String, WebDocument>,
    documents: &Vec<TypstDocument>,
    book: FontBook,
    fonts: Vec<Font>,
    url_path: Option<&str>,
) {
    for d in documents {
        let url = match url_path {
            Some(p) => &format!("{}/{}", p, d.url()),
            None => d.url(),
        };

        println!("exporting {}...", d.name());
        let pdf_url = export_document(d, book.clone(), fonts.clone());
        println!("... {} exported.", d.name());

        if map
            .insert(url.to_string(), WebDocument::new(d, pdf_url))
            .is_some()
        {
            panic!("The url {url} has occured multiple times");
        }

        match d.sub_documents() {
            Some(ds) => {
                export_documents(map, ds, book.clone(), fonts.clone(), Some(url));
            }
            None => (),
        };
    }
}

fn export_document(document: &TypstDocument, book: FontBook, fonts: Vec<Font>) -> String {
    export_html(document, book.clone(), fonts.clone());
    export_pdf(document, book, fonts)
}

fn export_pdf(document: &TypstDocument, book: FontBook, fonts: Vec<Font>) -> String {
    let docjob = Asgård::pdf(document, book.clone(), fonts.clone());
    let typed_doc = typst::compile(&docjob)
        .output
        .expect("Error compiling pdf version");

    let pdf = typst_pdf::pdf(&typed_doc, &PdfOptions::default()).expect("Error generating pdf");

    let path = format!("./{}/{}.pdf", PDF_DIRECTORY, document.filename_name());
    fs::write(path.clone(), pdf).expect("Error writing PDF.");
    path
}

fn export_html(document: &TypstDocument, book: FontBook, fonts: Vec<Font>) {
    let htmljob = Asgård::html(document, book, fonts);
    let typed_hmtl = typst::compile(&htmljob)
        .output
        .expect("Error compiling html version");

    let mut html = typst_html::html(&typed_hmtl).expect("Error generating html");
    html = format!("{}\n{}\n{}", TEMPLATE_PREPEND, html, TEMPLATE_APPEND);

    let path = format!(
        "./templates/{}{}.html.tera",
        HTML_DIRECTORY,
        document.filename_name()
    );
    fs::write(path, html).expect("Error writing html");
}
