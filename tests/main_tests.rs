use actix_web::{http::header::CONTENT_TYPE, test, web, App};
use projectf::dictionary::SimpleDictionary;
use projectf::metrics::AppState;
use serde_json::json;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[actix_web::test]
async fn test_explain_endpoint_stub_returns_ok_json() {
    let app_state = web::Data::new(AppState {
        counter: Arc::new(AtomicUsize::new(0)),
    });

    // Load the real dictionary from JSON file
    let dict_path = Path::new("data/medical_terms.json");
    let dict = web::Data::new(SimpleDictionary::load_from_json(
        dict_path.to_str().unwrap(),
    ));

    // Stub the explain handler to avoid hitting the real model
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .app_data(dict.clone())
            .route(
                "/explain",
                web::post().to(|req: web::Json<projectf::ExplainRequest>| async move {
                    let findings = req.findings.clone();
                    let fake_summary = format!("Summary of: {}", &findings);
                    let fake_highlight = format!("<p>Fake highlight: {}</p>", &findings);

                    web::Json(projectf::ExplainResponse {
                        patient_explanation: fake_summary,
                        highlighted_findings: fake_highlight,
                    })
                }),
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/explain")
        .insert_header((CONTENT_TYPE, "application/json"))
        .set_payload(json!({ "findings": "Lesion in the spleen" }).to_string())
        .to_request();

    let resp: projectf::ExplainResponse = test::call_and_read_body_json(&app, req).await;

    assert!(resp.patient_explanation.contains("Summary of:"));
    assert!(resp.highlighted_findings.contains("Fake highlight"));
}