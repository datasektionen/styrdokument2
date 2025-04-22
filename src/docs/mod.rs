mod exporter;
mod file_handler;
mod typst_wrapper;

use std::collections::HashMap;

use exporter::export;
pub use exporter::WebDocument;
use file_handler::get_documents;
pub use file_handler::TypstDocument;

pub fn setup() -> HashMap<String, WebDocument> {
    let documents = get_documents();
    export(&documents)
}
