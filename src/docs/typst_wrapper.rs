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
    source: HashMap<FileId, FileEntry>,
    fonts: Vec<Font>,
    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,
}

const MAIN: &str = "/main.typ";

impl Asgård {
    pub fn new(document: &Document) -> Self {
        let content = format!(
            r#"
#import "template.typ": *
#show: project.with(
    date: datetime.today().display(),
    title: "{}"
)
#include "{}"
git gud
            "#,
            document.title(),
            document.filename()
        );

        let mut sources = HashMap::new();
        let main = create_source(MAIN, content.clone());
        let main_entry = FileEntry::new(content.into(), Some(main.clone()));
        sources.insert(main.id(), main_entry);

        let styrdok_content = document.contents();
        let styrdok = create_source(document.filename(), styrdok_content.clone());
        let styrdok_entry = FileEntry::new(styrdok_content.into(), Some(styrdok.clone()));
        sources.insert(styrdok.id(), styrdok_entry);

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

    fn file_handler(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }
        let path = if let Some(_) = id.package() {
            // Fetching file from package
            unimplemented!("Packages not included")
        } else {
            // Fetching file from disk
            id.vpath().resolve(&self.root)
        }
        .ok_or(FileError::AccessDenied)?;

        let content = std::fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
        Ok(files
            .entry(id)
            .or_insert(FileEntry::new(content, None))
            .clone())
    }
}

/// A File that will be stored in the HashMap.
#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
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
        create_file_id(MAIN)
    }

    /// Try to access the specified source file.
    fn source(&self, id: FileId) -> FileResult<Source> {
        println!("Searching for {:?}", id);
        match self.source.get(&id) {
            Some(d) => {
                let mut d = d.clone();
                d.source(id)
            }
            None => self.file_handler(id)?.source(id),
        }
    }

    /// Try to access the specified file.
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file_handler(id).map(|file| file.bytes.clone())
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
    FileId::new(None, VirtualPath::new(filename))
}

fn create_fontbook() -> (FontBook, Vec<Font>) {
    let paths = fs::read_dir("typst/fonts/").expect("Could not find ./typst/fonts");

    let mut fonts = Vec::new();
    let mut fontbook = FontBook::new();
    for entry in paths {
        let entry = match entry {
            Ok(p) => p,
            _ => continue,
        };

        let path = entry.path();
        let data = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(_) => continue,
        };
        let font = match Font::new(Bytes::new(data.clone()), 0) {
            Some(f) => f,
            None => continue,
        };
        fonts.push(font);

        let info = FontInfo::new(data.as_slice(), 0).expect("Could not parse font");
        fontbook.push(info);
    }

    (fontbook, fonts)
}
