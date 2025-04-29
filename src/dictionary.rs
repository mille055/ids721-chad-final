use htmlescape::encode_attribute;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct MedicalTerm {
    pub term: String,
    #[allow(dead_code)]
    pub synonyms: Vec<String>,
    pub definition: String,
}

pub struct SimpleDictionary {
    terms: HashMap<String, String>,
}

impl SimpleDictionary {
    /// Load dictionary from a JSON file
    pub fn load_from_json(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Failed to read medical_terms.json");
        let terms_list: Vec<MedicalTerm> =
            serde_json::from_str(&file_content).expect("Failed to parse JSON");

        let mut terms = HashMap::new();
        for term_entry in terms_list {
            terms.insert(
                term_entry.term.to_lowercase(),
                term_entry.definition.clone(),
            );
            // We are **not** adding synonyms for now
        }

        Self { terms }
    }

    pub fn lookup(&self, term: &str) -> Option<&String> {
        self.terms.get(&term.to_lowercase())
    }

    pub fn all_terms(&self) -> Vec<String> {
        self.terms.keys().cloned().collect()
    }

    /// Highlight medical terms inside the text
    pub fn highlight_medical_terms(&self, text: &str) -> String {
        let mut placeholder_map = HashMap::new();
        let mut modified_text = text.to_string();

        let mut terms = self.all_terms();
        terms.sort_by(|a, b| b.len().cmp(&a.len())); // Match longer terms first

        let mut placeholder_index = 0;

        for term in terms {
            let re = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(&term))).unwrap();
            modified_text = re.replace_all(&modified_text, |caps: &regex::Captures| {
                let matched = caps.get(0).unwrap().as_str();
                let key = format!("%%HIGHLIGHT_{}%%", placeholder_index);
                placeholder_index += 1;
                if let Some(definition) = self.lookup(matched) {
                    let escaped_def = encode_attribute(definition);
                    let span = format!(
                        r#"<span title="{}" style="text-decoration: underline dotted; color: blue; font-weight: bold; cursor: help;">{}</span>"#,
                        escaped_def,
                        matched
                    );
                    placeholder_map.insert(key.clone(), span);
                    key
                } else {
                    matched.to_string()
                }
            }).to_string();
        }

        // Replace all placeholders with actual HTML
        for (key, value) in placeholder_map {
            modified_text = modified_text.replace(&key, &value);
        }

        modified_text.replace('\n', "<br>")
    }
}
