use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber;

use projectf::dictionary::SimpleDictionary;
use projectf::metrics::{metrics_endpoint, AppState};
use projectf::{ExplainRequest, ExplainResponse};

#[get("/")]
async fn home() -> impl Responder {
    let html = std::fs::read_to_string("static/home.html")
        .expect("Failed to read static/home.html");

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
    HttpResponse::Ok()
        .content_type(ContentType::html())
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

    let llama_req = projectf::LlamaRequest {
        prompt,
        n_predict: 100,
        temperature: 0.2,
        stream: false,
    };

    let llama_base_url =
        std::env::var("LLAMAFILE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let llama_completion_url = format!("{}/completion", llama_base_url);

    let res = client
        .post(&llama_completion_url)
        .json(&llama_req)
        .send()
        .await
        .expect("Failed to send request to llamafile");

    let response_text = res
        .text()
        .await
        .expect("Failed to read Llama response text");
    let llama_response: projectf::LlamaResponse =
        serde_json::from_str(&response_text).expect("Invalid response from Llama");

    let highlighted_input = dictionary.highlight_medical_terms(&req.findings);
    let cleaned_text = projectf::clean_llama_output(&llama_response.content);
    let patient_friendly = projectf::extract_before_triple_quotes(&cleaned_text);

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
