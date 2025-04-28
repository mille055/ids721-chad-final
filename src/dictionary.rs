use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use regex::Regex;
use htmlescape::encode_attribute; 
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct MedicalTerm {
    pub term: String,
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
        let terms_list: Vec<MedicalTerm> = serde_json::from_str(&file_content).expect("Failed to parse JSON");

        let mut terms = HashMap::new();
        for term_entry in terms_list {
            terms.insert(term_entry.term.to_lowercase(), term_entry.definition.clone());
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
        let mut output: String = text.to_string();

        let mut terms = self.all_terms();
        terms.sort_by(|a, b| b.len().cmp(&a.len())); // Match longer terms first

        for term in terms {
            // WARNING: No lookbehind (because Rust regex doesn't support it)
            let re = Regex::new(&format!(r#"(?i)\b{}\b"#, regex::escape(&term))).unwrap();

            output = re.replace_all(&output, |caps: &regex::Captures| {
                let word = caps.get(0).unwrap().as_str();
                if let Some(definition) = self.lookup(word) {
                    let escaped_def = encode_attribute(definition); // Escape for title attribute
                    format!(
                        r#"<span title="{}" style="text-decoration: underline dotted; color: blue; font-weight: bold; cursor: help;">{}</span>"#,
                        escaped_def,
                        word
                    )
                } else {
                    word.to_string()
                }
            }).to_string();
        }

        output.replace('\n', "<br>")
    }

}
