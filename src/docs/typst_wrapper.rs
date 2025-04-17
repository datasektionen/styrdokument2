use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use time;
use typst::{
    self,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
    Library,
};

use super::file_handler::Document;

pub struct Asgård {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    root: PathBuf,
    source: HashMap<FileId, Source>,
    fonts: Vec<Font>,
    files: Arc<Mutex<HashMap<FileId, Bytes>>>,
}

impl Asgård {
    pub fn new(document: &Document) -> Self {
        let content = format!(
            r#"
//#import "template.typ": *
//#show: project.with(
//    date: datetime.today().display(),
//    title: "{}"
//)
#include "../styrdokument/{}"
            "#,
            document.title(),
            document.filename()
        );

        let mut sources = HashMap::new();
        let main = create_source("/main.typ", content);
        sources.insert(main.id(), main);

        let (book, fonts) = create_fontbook();
        let root = PathBuf::from("./typst/");
        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            fonts,
            root,
            source: sources,
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn file_handler(&self, id: FileId) -> FileResult<Bytes> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }
        let path = if let Some(_) = id.package() {
            // Fetching file from package
            panic!("Packages not supported")
        } else {
            // Fetching file from disk
            id.vpath().resolve(&self.root)
        }
        .ok_or(FileError::AccessDenied)?;

        let content = std::fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
        Ok(files.entry(id).or_insert(Bytes::new(content)).clone())
    }
}

impl typst::World for Asgård {
    /// The standard library.
    ///
    /// Can be created through `Library::build()`.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known fonts.
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    /// Get the file id of the main source file.
    fn main(&self) -> FileId {
        self.source.id()
    }

    /// Try to access the specified source file.
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            //println!("id is {:?}", id);
            //unimplemented!()
            let f = self.file_handler(id)
        }
    }

    /// Try to access the specified file.
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file_handler(id)
    }

    /// Try to access the font with the given index in the font book.
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    /// Get the current date.
    ///
    /// If no offset is specified, the local date should be chosen. Otherwise,
    /// the UTC date should be chosen with the corresponding offset in hours.
    ///
    /// If this function returns `None`, Typst\'s `datetime` function will
    /// return an error.
    fn today(&self, _: Option<i64>) -> Option<Datetime> {
        let now = time::OffsetDateTime::now_utc().date();
        Some(Datetime::Date(now))
    }
}

fn create_source(filename: &str, content: String) -> Source {
    let file_id = create_file_id(filename);
    Source::new(file_id, content)
}

fn create_file_id(filename: &str) -> FileId {
    FileId::new_fake(VirtualPath::new(filename))
}

//fn create_source(document: &Document, path: String) -> Source {
//    let 
//}

fn create_fontbook() -> (FontBook, Vec<Font>) {
    let paths = fs::read_dir("typst/fonts/").expect("Could not find ./typst/fonts");

    let mut fonts = Vec::new();
    let mut fontbook = FontBook::new();
    for (i, entry) in paths.enumerate() {
        let entry = match entry {
            Ok(p) => p,
            _ => continue,
        };
        let i = i as u32;

        let path = entry.path();
        let data = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };
        let font = match Font::new(Bytes::new(data.clone()), i) {
            Some(f) => f,
            None => continue,
        };
        fonts.push(font);

        let info = FontInfo::new(data.as_slice(), i).expect("Could not parse font");
        fontbook.push(info);
    }

    (fontbook, fonts)
}
