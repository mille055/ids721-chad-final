use actix_web::{post, web, App, HttpServer, Responder, get, HttpResponse};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing_actix_web::TracingLogger;
use tracing_subscriber;
use tracing::info;
mod metrics;
use metrics::{AppState, metrics_endpoint};
mod dictionary;
use dictionary::SimpleDictionary;

#[derive(Deserialize)]
struct ExplainRequest {
    findings: String,
}

#[derive(Serialize)]
struct LlamaRequest {
    prompt: String,
    n_predict: usize,
    temperature: f32,
    stream: bool,
}

#[derive(Deserialize)]
struct LlamaResponse {
    content: String,

    #[serde(flatten)]
    _rest: serde_json::Value,
}

#[derive(Serialize)]
struct ExplainResponse {
    patient_explanation: String,
    highlighted_findings: String,
}

fn clean_llama_output(raw_output: &str) -> String {
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

fn extract_before_triple_quotes(text: &str) -> String {
    if let Some(index) = text.find("\"\"\"") {
        text[..index].trim().to_string()
    } else {
        text.trim().to_string()
    }
}

#[get("/")]
async fn home() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Patient-Friendly Radiology Report Converter</title>
        <script>
            async function submitForm(event) {
                event.preventDefault();
                const findings = document.getElementById('findings').value;

                // 1. Highlight terms immediately
                const highlightResponse = await fetch('/highlight', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ findings })
                });
                const highlightedText = await highlightResponse.text();
                document.getElementById('highlighted').innerHTML = highlightedText;

                // 2. Start loading for patient-friendly explanation
                document.getElementById('result').innerHTML = `<em>Loading patient-friendly explanation...</em>`;

                try {
                    const explainResponse = await fetch('/explain', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ findings })
                    });

                    if (!explainResponse.ok) {
                        throw new Error("Failed to get explanation");
                    }

                    const data = await explainResponse.json();
                    document.getElementById('result').innerText = data.patient_explanation;
                } catch (error) {
                    console.error('Error during explain fetch:', error);
                    document.getElementById('result').innerText = "Failed to generate explanation.";
                }
            }
        </script>
    </head>
    <body>
        <h1>Radiology Findings to Patient-Friendly Language</h1>
        <form onsubmit="submitForm(event)">
            <textarea id="findings" name="findings" rows="10" cols="80" placeholder="Paste radiology findings here..."></textarea><br><br>
            <button type="submit">Convert</button>
        </form>

        <br><br>
        <h2>Highlighted Findings (hover over terms):</h2>
        <p id="highlighted" style="border: 1px solid #ccc; padding: 10px;"></p>

        <br><br>
        <h2>Patient-Friendly Explanation:</h2>
        <p id="result" style="border: 1px solid #ccc; padding: 10px;"></p>
    </body>
    </html>
    "#;

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[post("/highlight")]
async fn highlight(
    req: web::Json<ExplainRequest>,
    dictionary: web::Data<SimpleDictionary>,
    ) -> impl Responder {
    let highlighted_input = dictionary.highlight_medical_terms(&req.findings);
    HttpResponse::Ok().
        content_type(ContentType::html())
        .body(highlighted_input)
}

#[post("/explain")]
async fn explain(
    req: web::Json<ExplainRequest>,
    app_state: web::Data<AppState>,
    dictionary: web::Data<SimpleDictionary>,
    ) -> impl Responder {
    let start_time = Instant::now();
    let client = Client::new();
    let prompt = format!(
        "You are a healthcare assistant.\n\n\
        Instructions:\n\
        - Rewrite the findings in simple, patient-friendly language.\n\
        - Output ONLY the rewritten explanation inside triple quotation marks (\"\"\").\n\
        - DO NOT explain your process.\n\
        - DO NOT include any other text outside the triple quotation marks.\n\
        - DO NOT speculate or add new information.\n\
        - DO NOT suggest that a doctor is making a diagnosis or assumption unless explicitly stated.\n\n\
        Example:\n\n\
        Findings:\n\
        Mild right pleural effusion.\n\n\
        Patient-friendly explanation:\n\
        \"\"\"\n\
        There is a small amount of extra fluid around your right lung.\n\
        \"\"\"\n\n\
        Findings:\n\
        {}\n\n\
        Patient-friendly explanation:\n\
        \"\"\"",
        req.findings
    );

    let llama_req = LlamaRequest {
        prompt,
        n_predict: 100,
        temperature: 0.2,
        stream: false,
    };

    let llama_base_url = std::env::var("LLAMAFILE_URL")
        .unwrap_or_else(|_| "http://host.docker.internal:8080".to_string());
    let llama_completion_url = format!("{}/completion", llama_base_url);

    let res = client
        .post(&llama_completion_url)
        .json(&llama_req)
        .send()
        .await
        .expect("Failed to send request to llamafile");

    let response_text = res.text().await.expect("Failed to read Llama response text");
    let llama_response: LlamaResponse = serde_json::from_str(&response_text)
        .expect("Invalid response from Llama");

    let highlighted_input = dictionary.highlight_medical_terms(&req.findings);
    let cleaned_text = clean_llama_output(&llama_response.content);
    let patient_friendly = extract_before_triple_quotes(&cleaned_text);

    let request_count = app_state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    let elapsed_time = start_time.elapsed();

    info!(
        "Processed request #{}, took {:.2?} seconds",
        request_count, elapsed_time
    );

    web::Json(ExplainResponse {
        patient_explanation: patient_friendly,
        highlighted_findings: highlighted_input,
    })
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    "OK"
}

#[get("/logtest")]
async fn logtest() -> impl Responder {
    info!("Logtest endpoint was called!");
    HttpResponse::Ok().body("Logtest complete. Check logs for info message.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("Starting Patient-Friendly Report Converter server at http://0.0.0.0:8000");

    let app_state = web::Data::new(AppState {
        counter: Arc::new(AtomicUsize::new(0)),
    });

    let medical_dictionary = SimpleDictionary::load_from_json("./data/medical_terms.json");
    let shared_dictionary = web::Data::new(medical_dictionary);

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(app_state.clone())
            .app_data(shared_dictionary.clone())
            .service(home)
            .service(explain)
            .service(highlight)
            .service(healthcheck)
            .service(metrics_endpoint)
            .service(logtest)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
