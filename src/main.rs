mod docs;
mod web;

use std::{env, fs};

use docs::{get_documents, Asgård};
use typst_pdf::PdfOptions;

fn main() {
    env::set_var("features", "html");
    let x = get_documents();
    println!("{:?}", x);
    let stadgar = &x[2];
    let pol = stadgar.sub_documents().unwrap();
    let dok = &pol[0];

    let docjob = Asgård::pdf(dok);
    let typed_doc = typst::compile(&docjob).output.expect("FUck compiling");

    let pdf = typst_pdf::pdf(&typed_doc, &PdfOptions::default()).expect("FFUKC export");
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf yo");

    let htmljob = Asgård::html(dok);
    let typed_hmtl = typst::compile(&htmljob).output.expect("html not compiling");

    let html = typst_html::html(&typed_hmtl).expect("html not exporting");
    fs::write("./output.html", html).expect("Error writing html.");
    println!("Created html yo");
}
