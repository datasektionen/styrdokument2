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

/// Rocket requires you to keep information which is to be kept in the state as `struct`s.
/// This will keep the [HashMap<String, WebDocument>] with the mapping from `url`s to the
/// [WebDocument]s, which are used to find the right `html` document, etc.
struct DocumentKeeper {
    hash: HashMap<String, WebDocument>,
}

/// Thing to append to the web page title.
const PAGE_TITLE_APPEND: &str = " - Datasektionens styrdokument";

/// The *home* page.
#[get("/")]
fn index() -> Template {
    Template::render(
        "home",
        context! {
            title: format!("{}{}", "Styrdokument", PAGE_TITLE_APPEND),
            name: "Styrdokument",
            pdf: "",
        },
    )
}

/// The `404` page.
#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "error",
        context! {
            title: format!("{}{}", "404", PAGE_TITLE_APPEND),
            name: "404",
            pdf: "",
        },
    )
}

/// Displays the styrdokument
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
            name: document.name(),
            pdf: format!("/{}", document.pdf_url()),
        },
    )
}

/// Workaround to always show the favicon.
#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/static/favicon.svg")
}

/// Main web function, which basically starts the Rocket server.
pub fn rocket(documents: HashMap<String, WebDocument>) -> Rocket<Build> {
    let spaceship = DocumentKeeper { hash: documents };

    rocket::build()
        .manage(spaceship) // mount the [DocumentKeeper]
        .attach(Template::fairing())
        .mount(
            // mount the file server which contains all pdf documents
            format!("/{}", PDF_DIRECTORY),
            FileServer::new(PDF_DIRECTORY, Options::None),
        )
        .mount("/static", FileServer::new("static", Options::None))
        .mount("/", routes![index, favicon, display_document])
}
