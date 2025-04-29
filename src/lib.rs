pub mod dictionary;
pub mod metrics;

pub use dictionary::{MedicalTerm, SimpleDictionary};
pub use metrics::AppState;

pub use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ExplainRequest {
    pub findings: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExplainResponse {
    pub patient_explanation: String,
    pub highlighted_findings: String,
}

#[derive(Serialize)]
pub struct LlamaRequest {
    pub prompt: String,
    pub n_predict: usize,
    pub temperature: f32,
    pub stream: bool,
}

#[derive(Deserialize)]
pub struct LlamaResponse {
    pub content: String,

    #[serde(flatten)]
    pub _rest: serde_json::Value,
}

/// Cleans output from the Llama model response by removing special tokens
pub fn clean_llama_output(raw_output: &str) -> String {
    raw_output
        .replace("<｜end▁of▁sentence｜>", "")
        .replace("</think>", "")
        .replace("<s>", "")
        .replace("</s>", "")
        .replace("\\n", " ")
        .replace("\n", " ")
        .trim()
        .trim_matches('"')
        .to_string()
}

/// Extracts only the text *before* the first set of triple quotes
pub fn extract_before_triple_quotes(text: &str) -> String {
    if let Some(index) = text.find("\"\"\"") {
        text[..index].trim().to_string()
    } else {
        text.trim().to_string()
    }
}
