mod docs;
mod web;

use std::fs;

use docs::{get_documents, Asgård};
use typst_pdf::PdfOptions;

fn main() {
    let x = get_documents();
    println!("{:?}", x);
    let stadgar = &x[2];
    let pol = stadgar.sub_documents().unwrap();
    let dok = &pol[0];

    let docjob = Asgård::new(dok);
    let typed_doc = typst::compile(&docjob).output.expect("FUck compiling");

    let pdf = typst_pdf::pdf(&typed_doc, &PdfOptions::default()).expect("FFUKC export");
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf yo");
}
