mod exporter;
mod file_handler;
mod typst_wrapper;

pub use file_handler::{get_documents, hashed_documents, Document};
pub use typst_wrapper::Asgård;

pub fn setup() {
    let documents = get_documents();
    println!("bung");
}
