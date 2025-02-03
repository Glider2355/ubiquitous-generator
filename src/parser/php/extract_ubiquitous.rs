use crate::parser::ubiquitous;

pub struct ExtractUbiquitousParam {
    pub class_name: String,
    pub doc_comment: String,
}

pub fn extract_ubiquitous(class_docs: Vec<ExtractUbiquitousParam>) -> Vec<ubiquitous::Ubiquitous> {
    class_docs
        .into_iter()
        .map(get_ubiquitous)
        .filter(|u| !u.is_all_none())
        .collect()
}

fn get_ubiquitous(class_doc: ExtractUbiquitousParam) -> ubiquitous::Ubiquitous {
    let comment = class_doc.doc_comment.trim();
    let mut result = ubiquitous::Ubiquitous::new();

    result = result.set_class_name(class_doc.class_name);

    for line in comment.lines() {
        let line = line.trim();

        // @ubiquitous
        if let Some(pos) = line.find("@ubiquitous") {
            let tag_len = "@ubiquitous".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_ubiquitous(val);
            }
        }
        // @context
        else if let Some(pos) = line.find("@context") {
            let tag_len = "@context".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_context(val);
            }
        }
        // @description
        else if let Some(pos) = line.find("@description") {
            let tag_len = "@description".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_description(val);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::ubiquitous::Ubiquitous;
    use super::*;

    #[test]
    fn test_extract_ubiquitous_empty() {
        let doc_comments = vec![];

        let result = extract_ubiquitous(doc_comments);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_ubiquitous_single() {
        let class_docs = vec![ExtractUbiquitousParam {
            class_name: "class_name".to_string(),
            doc_comment: r#"/**
    * @ubiquitous ubiquitous_lang
    */"#
            .to_string(),
        }];
        let result = extract_ubiquitous(class_docs);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            Ubiquitous::new()
                .set_class_name("class_name".to_string())
                .set_ubiquitous("ubiquitous_lang".to_string())
        );
    }

    #[test]
    fn test_extract_ubiquitous_multi_fields() {
        let class_docs = vec![ExtractUbiquitousParam {
            class_name: "class_name".to_string(),
            doc_comment: r#"/**
    * @ubiquitous ubiquitous_lang
    * @context context_example
    * @description description_text
    */"#
            .to_string(),
        }];
        let result = extract_ubiquitous(class_docs);
        assert_eq!(result.len(), 1);

        let expected = Ubiquitous::new()
            .set_class_name("class_name".to_string())
            .set_ubiquitous("ubiquitous_lang".to_string())
            .set_context("context_example".to_string())
            .set_description("description_text".to_string());
        assert_eq!(result[0], expected);
    }
}
