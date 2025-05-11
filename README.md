# styrdokument2
En samling av Konglig Datasektionens formella dokument - nu rostiga

## Description

This is a collection of the regulatory and governing documents for the organization Konglig Datasektionen. Besides hosting the documents it runs as a web server to serve `html` and `pdf` versions of the documents for eazy viewing.

All documents are written in the `typst` type-setting language, which are compiled to both `html` and `pdf` versions.

## How to use

### Running

The web server is entirely self contained and just requires `Rust` and `Cargo`. Simply run

```sh
cargo run .
```

to run the web server locally.

### Editing the styrdokument

Since the repo contains both the web server and the `typst` documents with the raw styrdokument code it can be a little daunting to know where to start. Luckily for you it's not so hard in reality. Since you reading this is most likely a member of the board I can tell you that it's much easier than having to deal with some of your other tasks (no specifics mentioned).

Unless you want to change something with the server the only areas where you need to bother checking out are the `./styrdokument/` and the `./typst/` directories. The `./styrdokument/` one contains all the actual styrdokument which you'll most likely want to change, and the `./typst/` contains the template code for the `pdf`s.

All styrdokuments are defined in two places. First of all there is the `{styrdokument}.typ` file in the `./styrdokument/` directory which contains the `typst` code for the document. Secondly, and perhaps the most important all documents need to be defined in the `./styrdokument/styrdokument.toml` file. This file tells the server what styrdokuments exists, their necessary information and what order they should be displayed in. Note that the server supports document hierarchy of depth 2, meaning that you can have a category of styrdokument with multiple separate documents inside.

#### Add styrdokument

1. Add the styrdokument information to the `./styrdokument/styrdokument.toml` file.
2. Add the `{styrdokument}.typ` file to the location defined in the `toml` file. Keep in mind that if you've defined the styrdokument as a `sub_document` it will assume that the location is within the above `directory` directory.

#### Edit styrdokument

If you want to change the contents of the document simply change the `.typ` file. Otherwise look towards the `styrdokument.toml` file.

#### Remove styrdokument


1. Remove the styrdokument information from the `./styrdokument/styrdokument.toml` file.
2. Remove the `{styrdokument}.typ` file from the location defined in the `toml` file. Keep in mind that if you're removing a category of documents you'll need to keep in mind that it will affect the sub documents.
