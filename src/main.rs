use actix_web::{post, web, App, HttpServer, Responder, get, HttpResponse};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Deserialize)]
struct ExplainRequest {
    findings: String,
}

#[derive(Serialize)]
struct LlamaRequest {
    prompt: String,
    n_predict: usize,
    temperature: f32,
}

#[derive(Deserialize)]
struct LlamaResponse {
    content: String,
}

#[derive(Serialize)]
struct ExplainResponse {
    patient_explanation: String,
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
                
                const response = await fetch('/explain', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ findings })
                });

                const data = await response.json();
                document.getElementById('result').innerText = data.patient_explanation;
            }
        </script>
    </head>
    <body>
        <h1>Radiology Findings to Patient-Friendly Language</h1>
        <form onsubmit="submitForm(event)">
            <textarea id="findings" name="findings" rows="10" cols="80" placeholder="Paste radiology findings here..."></textarea><br><br>
            <button type="submit">Convert</button>
        </form>
        <br>
        <h2>Patient-Friendly Explanation:</h2>
        <p id="result"></p>
    </body>
    </html>
    "#;

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}


#[post("/explain_form")]
async fn explain_form(form: web::Form<ExplainRequest>) -> impl Responder {
    let client = Client::new();

    let prompt = format!(
        "You are a healthcare assistant.\n\n\
        Instructions:\n\
        - Rewrite the findings in simple, patient-friendly language.\n\
        - Output ONLY the rewritten explanation inside triple quotation marks (\"\"\").\n\
        - Do NOT explain your process.\n\
        - Do NOT include any other text outside the triple quotation marks.\n\n\
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
        form.findings
    );

    let llama_req = LlamaRequest {
        prompt,
        n_predict: 250,
        temperature: 0.2,
    };

    let res = client.post("http://llama:8080/completion")
        .json(&llama_req)
        .send()
        .await
        .expect("Failed to send request to llamafile");

    let llama_response: LlamaResponse = res.json().await.expect("Invalid response");

    let cleaned_text = clean_llama_output(&llama_response.content);
    println!("RAW Llama cleaned output:\n{}", cleaned_text); 
    let patient_friendly = extract_before_triple_quotes(&cleaned_text);

    let result_html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Patient-Friendly Explanation</title>
        </head>
        <body>
            <h1>Converted Explanation:</h1>
            <p>{}</p>
            <br>
            <a href="/">Go back</a>
        </body>
        </html>
        "#,
        patient_friendly
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(result_html)
}

#[post("/explain")]
async fn explain(req: web::Json<ExplainRequest>) -> impl Responder {
    let client = Client::new();

    let prompt = format!(
        "You are a healthcare assistant.\n\n\
        Instructions:\n\
        - Rewrite the findings in simple, patient-friendly language.\n\
        - Output ONLY the rewritten explanation inside triple quotation marks (\"\"\").\n\
        - Do NOT explain your process.\n\
        - Do NOT include any other text outside the triple quotation marks.\n\n\
        - DO NOT speculate or add new information.
        - DO NOT suggest that a doctor is making a diagnosis or assumption unless explicitly stated.
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
        n_predict: 250,
        temperature: 0.2,
    };

    let res = client.post("http://llama:8080/completion")
        .json(&llama_req)
        .send()
        .await
        .expect("Failed to send request to llamafile");

    let llama_response: LlamaResponse = res.json().await.expect("Invalid response");

    let cleaned_text = clean_llama_output(&llama_response.content);
    println!("RAW Llama cleaned output:\n{}", cleaned_text);
    let patient_friendly = extract_before_triple_quotes(&cleaned_text);

    web::Json(ExplainResponse {
        patient_explanation: patient_friendly,
    })
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Patient-Friendly Report Converter server at http://0.0.0.0:8000");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(explain)
            .service(explain_form)
            .service(healthcheck)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
