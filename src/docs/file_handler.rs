use std::fs;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct Document {
    name: String,
    filename: String,
    url: String,
    directory: Option<String>,
    sub_documents: Option<Vec<Document>>,
}

pub fn get_documents() -> Vec<Document> {
    let content = fs::read_to_string("styrdokument/styrdokument.toml")
        .expect("Failed to read styrdokument.toml");
    parse_styrdokument_toml(content)
}

#[derive(serde::Deserialize, Clone, Debug)]
struct DocumentWrapper {
    documents: Vec<Document>,
}

fn parse_styrdokument_toml(toml_content: String) -> Vec<Document> {
    let dw: DocumentWrapper =
        toml::from_str(&toml_content).expect("Failed to parse styrdokument.toml");
    dw.documents
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Document {
                name: "Stadgar".to_string(),
                filename: "stadgar.typ".to_string(),
                url: "stadgar".to_string(),
                directory: None,
                sub_documents: None,
            },
            Document {
                name: "Reglemente".to_string(),
                filename: "reglemente.typ".to_string(),
                url: "reglemente".to_string(),
                directory: None,
                sub_documents: None,
            },
            Document {
                name: "Policies".to_string(),
                filename: "policies.typ".to_string(),
                url: "policies".to_string(),
                directory: Some("policies".to_string()),
                sub_documents: Some(vec![
                    Document {
                        name: "Uppförandepolicy".to_string(),
                        filename: "uppförandepolicy.typ".to_string(),
                        url: "uppforandepolicy".to_string(),
                        directory: None,
                        sub_documents: None,
                    },
                    Document {
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
