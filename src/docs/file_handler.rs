use std::fs;

/// A regularory [Document] (styrdokument). The struct contains
/// the official `name` of the document, the `filename` of the document,
/// which has to be a `.typ` (typst) file for the rest of the program
/// to function. The `url` field specifies what url should lead to this
/// document.
///
/// The `directory` and `sub_documents` fields are there in case the "document"
/// is actually a collection of documents, for example all the policies. Both need
/// to be used if this is the case, as the `directory` field is needed to find the
/// sub_documents.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Document {
    name: String,
    filename: String,
    url: String,
    path: String,
    directory: Option<String>, // needed if the document has sub documents
    sub_documents: Option<Vec<Document>>, // if the "document" is actually a directory
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, PartialOrd)]
struct Intermediary {
    name: String,
    filename: String,
    url: String,
    directory: Option<String>, // needed if the document has sub documents
    sub_documents: Option<Vec<Intermediary>>, // if the "document" is actually a directory
}

impl Document {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn title(&self) -> &str {
        self.name()
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn full_path(&self) -> String {
        format!("styrdokument/{}/{}", self.path, self.filename)
    }

    pub fn contents(&self) -> String {
        std::fs::read_to_string(self.full_path()).expect("Failed to read document contents")
    }

    fn from_intermediary(value: &Intermediary, path: String) -> Self {
        let sub_documents = match &value.sub_documents {
            Some(sd) => {
                let next_path = format!("{}/{}", path, value.directory.clone().unwrap());
                let res = sd
                    .iter()
                    .map(|d| Document::from_intermediary(&d, next_path.clone()))
                    .collect();
                Some(res)
            }
            None => None,
        };
        Document {
            name: value.name.clone(),
            filename: value.filename.clone(),
            url: value.url.clone(),
            path,
            directory: value.directory.clone(),
            sub_documents,
        }
    }
}

pub fn get_documents() -> Vec<Document> {
    let content = fs::read_to_string("styrdokument/styrdokument.toml")
        .expect("Failed to read styrdokument.toml");
    let intermidiary_documents = parse_styrdokument_toml(content);
    let docs = intermidiary_documents
        .iter()
        .map(|d| Document::from_intermediary(d, "".to_string()))
        .collect();
    docs
}

/// Wrapper for creating a [Vec<Document>].
#[derive(serde::Deserialize, Clone, Debug)]
struct DocumentWrapper {
    documents: Vec<Intermediary>,
}

/// Parses a [toml] [String] to find an array of [Document]s.
/// The [toml] [String] has to be of the form:
///
/// [[documents]]
/// name = "name1"
/// filename = "filename1.typ"
/// url = "url1"
///
/// [[documents]]
/// name = "name2"
/// filename = "filename2.typ"
/// url = "url2"
/// directory = "dir1"
///
/// [[documents.sub_documents]]
/// name = "name3"
/// filename = "filename2.typ"
/// url = "url3"
fn parse_styrdokument_toml(toml_content: String) -> Vec<Intermediary> {
    let dw: DocumentWrapper =
        toml::from_str(&toml_content).expect("Failed to parse styrdokument.toml");
    dw.documents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_styrdokument_toml() {
        let _ = get_documents();
    }
    #[test]
    #[should_panic]
    fn test_parse_styrdokument_toml_incorrect() {
        let content = "
            [[documents]]
            shame = \"Stadgar\"
            filename = \"stadgar.typ\"
            url = \"stadgar\"
        ";

        parse_styrdokument_toml(content.to_string());
    }

    #[test]
    fn test_parse_styrdokument_toml_correct() {
        let content = "
            [[documents]]
            name = \"Stadgar\"
            filename = \"stadgar.typ\"
            url = \"stadgar\"


            [[documents]]
            name = \"Reglemente\"
            filename = \"reglemente.typ\"
            url = \"reglemente\"

            [[documents]]
            name = \"Policies\"
            filename = \"policies.typ\"
            url = \"policies\"
            directory = \"policies\"

            [[documents.sub_documents]]
            name = \"Uppförandepolicy\"
            filename = \"uppförandepolicy.typ\"
            url = \"uppforandepolicy\"

            [[documents.sub_documents]]
            name = \"Samarbetspolicy\"
            filename = \"samarbetspolicy.typ\"
            url = \"samarbetspolicy\"
        ";

        let expected = vec![
            Intermediary {
                name: "Stadgar".to_string(),
                filename: "stadgar.typ".to_string(),
                url: "stadgar".to_string(),
                directory: None,
                sub_documents: None,
            },
            Intermediary {
                name: "Reglemente".to_string(),
                filename: "reglemente.typ".to_string(),
                url: "reglemente".to_string(),
                directory: None,
                sub_documents: None,
            },
            Intermediary {
                name: "Policies".to_string(),
                filename: "policies.typ".to_string(),
                url: "policies".to_string(),
                directory: Some("policies".to_string()),
                sub_documents: Some(vec![
                    Intermediary {
                        name: "Uppförandepolicy".to_string(),
                        filename: "uppförandepolicy.typ".to_string(),
                        url: "uppforandepolicy".to_string(),
                        directory: None,
                        sub_documents: None,
                    },
                    Intermediary {
                        name: "Samarbetspolicy".to_string(),
                        filename: "samarbetspolicy.typ".to_string(),
                        url: "samarbetspolicy".to_string(),
                        directory: None,
                        sub_documents: None,
                    },
                ]),
            },
        ];

        let res = parse_styrdokument_toml(content.to_string());
        assert_eq!(expected, res)
    }
}
