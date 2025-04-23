use crate::docs::{WebDocument, HTML_DIRECTORY, PDF_DIRECTORY};
use rocket::{
    catch,
    fs::{FileServer, Options},
    get,
    response::Redirect,
    routes, Build, Rocket, State,
};
use rocket_dyn_templates::{context, Template};
use std::{collections::HashMap, path::PathBuf};

struct DocumentKeeper {
    hash: HashMap<String, WebDocument>,
}

const PAGE_TITLE_APPEND: &str = " - Datasektionens styrdokument";

#[get("/")]
fn index() -> Template {
    Template::render(
        "home",
        context! {
            title: format!("{}{}", "Styrdokument", PAGE_TITLE_APPEND),
        },
    )
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "error",
        context! {
            title: format!("{}{}", "404", PAGE_TITLE_APPEND),
        },
    )
}

#[get("/dokument/<name..>")]
fn display_document(name: PathBuf, document_keeper: &State<DocumentKeeper>) -> Template {
    let url = name.to_str().unwrap().to_string();
    let document = match document_keeper.hash.get(&url) {
        Some(d) => d,
        None => return not_found(),
    };

    Template::render(
        format!("{}{}", HTML_DIRECTORY, document.filename()),
        context! {
            title: format!("{}{}", document.name(), PAGE_TITLE_APPEND),
        },
    )
}

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/static/favicon.svg")
}

pub fn rocket(documents: HashMap<String, WebDocument>) -> Rocket<Build> {
    let spaceship = DocumentKeeper { hash: documents };

    rocket::build()
        .manage(spaceship)
        .attach(Template::fairing())
        .mount(
            format!("/{}", PDF_DIRECTORY),
            FileServer::new(PDF_DIRECTORY, Options::None),
        )
        .mount("/static", FileServer::new("static", Options::None))
        .mount("/", routes![index, favicon, display_document])
}
