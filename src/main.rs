mod docs;
mod web;

use docs::get_documents;

fn main() {
    let x = get_documents();
    println!("{:?}", x);
}
