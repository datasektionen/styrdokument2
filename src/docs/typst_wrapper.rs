use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

use time;
use typst::{
    self,
    foundations::Bytes,
    syntax::{FileId, Source},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
    Library,
};

pub struct SmallWorld {
    library: Library,
    book: LazyHash<FontBook>,
    main: FileId,
    source: Source,
    files: Arc<Mutex<HashMap<FileId, Bytes>>>,
}

fn create_fontbook() -> FontBook {
    let paths = fs::read_dir("typst/fonts/").expect("Could not find ./typst/fonts");

    let mut fontbook = FontBook::new();
    let mut index = 0;
    for entry in paths {
        let entry = match entry {
            Ok(p) => p,
            _ => continue,
        };

        let path = entry.path();
        let data = fs::read_to_string(path).expect("Error reading font");
        let data = data.as_bytes();

        let info = FontInfo::new(data, index).expect("Could not parse font");
        fontbook.push(info);
        index += 1;
    }

    fontbook
}

//impl SmallWorld {
//    pub fn new() -> Self {
//        let fonts = create_fontbook();
//        Self {
//            library: Library::default(),
//            book: LazyHash::new(fonts),
//            main: (),
//            source: (),
//            files: (),
//        }
//    }
//}
