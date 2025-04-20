use crate::docs::{hashed_documents, Document};
use rocket::{get, routes, Build, Rocket, State};
use std::{collections::HashMap, path::PathBuf};

struct Spaceship {
    hash: HashMap<String, Document>,
}

#[get("/")]
fn index() -> String {
    "Yo yo yo".to_string()
}

#[get("/<name..>")]
fn hello(name: PathBuf, spaceship: &State<Spaceship>) -> String {
    let url = name.to_str().unwrap().to_string();
    let doc = spaceship.hash.get(&url);
    let thing = match doc {
        Some(d) => d.name(),
        None => "fuck",
    };

    format!("Hello, {}!", thing)
}

pub fn rocket(documents: Vec<Document>) -> Rocket<Build> {
    let document_map = hashed_documents(documents);

    let spaceship = Spaceship { hash: document_map };

    rocket::build()
        .manage(spaceship)
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
