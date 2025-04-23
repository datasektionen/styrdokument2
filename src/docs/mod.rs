mod exporter;
mod file_handler;
mod typst_wrapper;

use std::collections::HashMap;

use exporter::export;
pub use exporter::{WebDocument, HTML_DIRECTORY, PDF_DIRECTORY};
use file_handler::get_documents;
pub use file_handler::TypstDocument;

pub fn setup() -> HashMap<String, WebDocument> {
    let documents = get_documents();
    export(&documents)
}
