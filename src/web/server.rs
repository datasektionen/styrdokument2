use crate::docs::{hashed_documents, Document};
use rocket::{get, routes, Build, Rocket, State};
use rocket_dyn_templates::{context, Template};
use std::{collections::HashMap, path::PathBuf};

struct Spaceship {
    hash: HashMap<String, Document>,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            document: "output.html"
        },
    )
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
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
