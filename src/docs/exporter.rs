use std::{
    collections::HashMap,
    fs::{self, create_dir, remove_dir_all, File},
    io::{BufWriter, Write},
};

use typst::text::{Font, FontBook};
use typst_pdf::PdfOptions;

use super::{
    typst_wrapper::{create_fontbook, Asgård},
    TypstDocument,
};

/// The directory where the exported `html` documents will be located in the `./templates`
/// directory.
pub const HTML_DIRECTORY: &str = "documents/";
/// The directory where the exported `pdf` documents will be located.
pub const PDF_DIRECTORY: &str = "public";
/// A string which is to be prepended to all `html` documents for them to function as `tera`
/// templates.
const TEMPLATE_PREPEND: &str = r#"{% extends "index" %}{% block content %}"#;
/// A string which is to be appended to all `html` documents for them to function as `tera`
/// templates.
const TEMPLATE_APPEND: &str = r#"{% endblock content %}"#;

/// A [WebDocument] is essentially a smaller version of the [TypstDocument] which only contains the
/// necessary information for the web server to function. Separating them is due to performance,
/// and partly to offer more customizability in the future to include more information pertaining
/// to the final documents.
#[derive(Clone)]
pub struct WebDocument {
    name: String,
    filename: String,
    pdf_url: String,
}

/// A [NavDocument] is an intermediary document used to create the right hand navbar on the
/// website. It's similar but separated from the [WebDocument] as it keeps the original nested
/// structure from the [TypstDocument]s.
struct NavDocument {
    name: String,
    url: String,
    sub_documents: Option<Vec<NavDocument>>,
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

/// Exports a list of [TypstDocument]s to their final `html` and `pdf` documents, and returns a
/// mapping from their `url`s to the [WebDocument]s that contain the relevant information about
/// them.
pub fn export(documents: &Vec<TypstDocument>) -> HashMap<String, WebDocument> {
    let (book, fonts) = create_fontbook();
    let mut document_mapping = HashMap::new();

    prepare_export_dirs(); // clean up export directories
    let nav_documents = export_documents(&mut document_mapping, documents, book, fonts, None);
    generate_side_navbar(nav_documents); // generate the html code for the right side navbar

    document_mapping
}

/// Recursively exports the documents to make sure that the hierarchical structure is preserved for
/// the urls.
fn export_documents(
    map: &mut HashMap<String, WebDocument>,
    documents: &Vec<TypstDocument>,
    book: FontBook,
    fonts: Vec<Font>,
    url_path: Option<&str>,
) -> Vec<NavDocument> {
    let mut nav_documents = Vec::new();
    for d in documents {
        let url = match url_path {
            Some(p) => &format!("{}/{}", p, d.url()),
            None => d.url(),
        };

        println!("exporting {}...", d.name());
        let pdf_url = export_document(d, book.clone(), fonts.clone());
        println!("...{} exported", d.name());

        if map
            .insert(url.to_string(), WebDocument::new(d, pdf_url))
            .is_some()
        {
            panic!("The url {url} has occured multiple times");
        }

        let sub_docs = d
            .sub_documents()
            .map(|ds| export_documents(map, ds, book.clone(), fonts.clone(), Some(url)));

        nav_documents.push(NavDocument {
            name: d.name().to_string(),
            url: format!("/dokument/{}", url),
            sub_documents: sub_docs,
        });
    }

    nav_documents
}

/// Exports a single [TypstDocument] to it's `html` and `pdf` variant.
fn export_document(document: &TypstDocument, book: FontBook, fonts: Vec<Font>) -> String {
    export_html(document, book.clone(), fonts.clone());
    export_pdf(document, book, fonts)
}

/// Export a [TypstDocument] to a `pdf` document.
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

/// Export a [TypstDocument] to a `html` document.
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

/// Deletes the export directories along with their contents and then creates new empty directories
/// to ensure that these exist for new exported content.
fn prepare_export_dirs() {
    let html_path = "./templates/documents";
    let _ = remove_dir_all(html_path);
    create_dir(html_path).unwrap_or_else(|_| panic!("Could not create {}", html_path));

    let pdf_path = &format!("./{}", PDF_DIRECTORY);
    let _ = remove_dir_all(pdf_path);
    create_dir(pdf_path).unwrap_or_else(|_| panic!("Could not create {}", pdf_path));
}

/// Generates the the `tera` template for the right hand navbar.
fn generate_side_navbar(nav_documents: Vec<NavDocument>) {
    let path = "./templates/navbar.html.tera";
    let file = File::create(path).expect("Cannot create navbar template");
    let mut writer = BufWriter::new(file);

    // The `{{% if "{}" == name %}}` code is meant to highlight the text if you're currently on
    // that page.
    for d in nav_documents {
        if d.sub_documents.is_none() {
            writeln!(
                &mut writer,
                r#"
<ul>
  <li>
    <a {{% if "{}" == name %}} class="text-theme-color strong" {{% endif %}} href="{}">{}</a>
  </li>
</ul>"#,
                d.name, d.url, d.name
            )
            .unwrap();
        } else {
            writeln!(
                &mut writer,
                r#"
<h3 style="margin-top: 1rem;">
  <a {{% if "{}" == name %}} class="text-theme-color strong" {{% endif %}} href="{}">{}</a>
</h3>"#,
                d.name, d.url, d.name
            )
            .unwrap();

            for ds in d.sub_documents.unwrap() {
                writeln!(
                    &mut writer,
                    r#"
<ul>
<li style="margin-left: 2rem;">
  <a {{% if "{}" == name %}} class="text-theme-color strong" {{% endif %}} href="{}">{}</a>
</li>
</ul>"#,
                    ds.name, ds.url, ds.name
                )
                .unwrap();
            }
            writeln!(&mut writer, r#"<p style="margin-bottom: 1rem;"/>"#,).unwrap();
        }
    }
}
