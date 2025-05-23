// A wrapper for using and implementing the needed typst functions. Much of this code has been
// inspired by, and in some cases completely stolen from,
// [https://github.com/tfachmann/typst-as-library](https://github.com/tfachmann/typst-as-library).

use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    vec,
};

use typst::{
    self,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
    Feature, Features, Library, LibraryBuilder,
};

use super::file_handler::TypstDocument;

/// A typst "[World]", but you know it's a bit abstract and hard to fully understand - almost like
/// the gods (don't think too much about it. I couldn't come up with a funnier name). Either way
/// the typst [World] is basically the entirety of the project and therefore has to contain
/// everything typst will use for compiling the documents.
///
/// * The `library` field contains the standard typst library meaning all the functions and stuff.
/// * `book` is the [FontBook], which more or less is an index of all available fonts in the project.
/// * `root` is the [PathBuf] root of the project, meaning where typst will search for all files
///   declared in a document. This includes images, documents etc.
/// * `source` contains the source files for the project, and in this case it's the files that do
///   not exist in the root. This includes the `main` file which is declared below, and the
///   styrdokument file which is outside of the `root` scope.
/// * `fonts` is simply a [Vec<Font>] which also contains all font data, which is indexed by the
///   [FontBook].
/// * `files` contains all files that have been found in the project. Don't think too much about
///   it.
pub struct Asgård<'a> {
    library: LazyHash<Library>,
    book: &'a LazyHash<FontBook>,
    root: PathBuf,
    source: HashMap<FileId, FileEntry>,
    fonts: &'a Vec<Font>,
    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,
}

/// `MAIN` contains the "filename" of the main file, which in typst **has** to be `/main.typ`.
const MAIN: &str = "/main.typ";
/// The absolute path to where the typst templating files are stored.
const DOCUMENT_PATH: &str = "./typst/";

impl<'a> Asgård<'a> {
    /// Creates a typst [World] intended for `pdf` output. This will include document formatting.
    pub fn pdf(
        document: &TypstDocument,
        book: &'a LazyHash<FontBook>,
        fonts: &'a Vec<Font>,
    ) -> Self {
        let content = format!(
            r#"
#import "template.typ": *
#show: project.with(
    date: datetime.today().display(),
    title: "{}"
)
#include "{}"
            "#,
            document.title(),
            document.filename()
        );

        Self::new(document, content, book, fonts)
    }

    /// Creates a typst [World] intended for `html` output. This does not include any formatting.
    pub fn html(
        document: &TypstDocument,
        book: &'a LazyHash<FontBook>,
        fonts: &'a Vec<Font>,
    ) -> Self {
        let content = format!(
            r#"
#include "{}"
            "#,
            document.filename(),
        );

        Self::new(document, content, book, fonts)
    }

    /// Creates a new [Asgård] typst [World].
    fn new(
        document: &TypstDocument,
        content: String,
        book: &'a LazyHash<FontBook>,
        fonts: &'a Vec<Font>,
    ) -> Self {
        let mut sources = HashMap::new();
        let main = create_source(MAIN, content.clone());
        let main_entry = FileEntry::new(content.into(), Some(main.clone()));
        sources.insert(main.id(), main_entry); // add the very short "main" document

        let styrdok_content = document.contents();
        let styrdok = create_source(document.filename(), styrdok_content.clone());
        let styrdok_entry = FileEntry::new(styrdok_content.into(), Some(styrdok.clone()));
        sources.insert(styrdok.id(), styrdok_entry); // add the styrdokument

        let root = PathBuf::from(DOCUMENT_PATH);

        let lib = asgård_library();
        Self {
            library: LazyHash::new(lib),
            book,
            fonts,
            root,
            source: sources,
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Handles importing files like documents and images from the `root` directory.
    fn file_handler(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }
        let path = if id.package().is_some() {
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

/// Kind of the only way to enable [Html] output while it's in the experimental phase.
/// TODO:
/// Clean up and move from experimental flag when the html export feature from typst is finished.
fn asgård_library() -> Library {
    let feature = vec![Feature::Html];
    let features: Features = Features::from_iter(feature);
    let builder = LibraryBuilder::default();
    let real_builder = builder.with_features(features);

    real_builder.build()
}

/// A [File] that will be stored in the HashMap.
#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    /// Creates a new [FileEntry], who would have thought?
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    /// Gets the required file from a [FileId], preferably already from the [Source], but if it
    /// has not already been added to the file pool of the [World] it will try to get it and then
    /// add it to the file pool.
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

impl typst::World for Asgård<'_> {
    /// The standard library.
    ///
    /// Can be created through `Library::build()`.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known fonts.
    fn book(&self) -> &LazyHash<FontBook> {
        self.book
    }

    /// Get the file id of the main source file.
    fn main(&self) -> FileId {
        create_file_id(MAIN)
    }

    /// Try to access the specified source file.
    fn source(&self, id: FileId) -> FileResult<Source> {
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

/// Creates a [Source] based on a document.
fn create_source(filename: &str, content: String) -> Source {
    let file_id = create_file_id(filename);
    Source::new(file_id, content)
}

/// Creates a [FileId] based on a filename.
fn create_file_id(filename: &str) -> FileId {
    FileId::new(None, VirtualPath::new(filename))
}

/// Creates a [FontBook] which indexes the [Vec<Font>].
pub fn create_fontbook() -> (LazyHash<FontBook>, Vec<Font>) {
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
        // For some unknown reason the `index` only works when it's `0`.
        let font = match Font::new(Bytes::new(data.clone()), 0) {
            Some(f) => f,
            None => continue,
        };
        fonts.push(font);

        let info = FontInfo::new(data.as_slice(), 0).expect("Could not parse font");
        fontbook.push(info);
    }

    (LazyHash::new(fontbook), fonts)
}
