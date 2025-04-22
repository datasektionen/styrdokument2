use crate::docs::WebDocument;
use rocket::{get, routes, Build, Rocket, State};
use rocket_dyn_templates::{context, Template};
use std::{collections::HashMap, path::PathBuf};

struct Spaceship {
    hash: HashMap<String, WebDocument>,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "output",
        context! {
            title: "i love styr",
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

pub fn rocket(documents: HashMap<String, WebDocument>) -> Rocket<Build> {
    let spaceship = Spaceship { hash: documents };

    rocket::build()
        .manage(spaceship)
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
