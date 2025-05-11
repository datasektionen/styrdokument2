mod docs;
mod web;

use web::rocket;

/// Your standard main function. Will first generate all needed documents and then launch the
/// Rocket server.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let document_mapping = docs::setup();

    let rocket = rocket(document_mapping);
    rocket.launch().await?;
    Ok(())
}
