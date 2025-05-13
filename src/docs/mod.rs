mod exporter;
mod file_handler;
mod fuzzyfile;
mod typst_wrapper;

use std::collections::HashMap;

use exporter::export;
pub use exporter::{WebDocument, HTML_DIRECTORY, PDF_DIRECTORY};
use file_handler::get_documents;
pub use file_handler::TypstDocument;

/// Sets up the pdf and html documents and generates the mapping from urls to actual documents.
pub fn setup() -> HashMap<String, WebDocument> {
    let documents = get_documents();
    export(&documents)
}
