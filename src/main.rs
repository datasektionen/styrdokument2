mod docs;
mod web;

use web::rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let document_mapping = docs::setup();

    let rocket = rocket(document_mapping);
    rocket.launch().await?;
    Ok(())
}
