use actix_web::{get, web, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct AppState {
    pub counter: Arc<AtomicUsize>,
}

#[get("/metrics")]
pub async fn metrics_endpoint(app_state: web::Data<AppState>) -> impl Responder {
    let request_count = app_state.counter.load(Ordering::Relaxed);

    let body = format!(
        "# HELP patient_converter_requests_total Total number of processed explain requests\n\
         # TYPE patient_converter_requests_total counter\n\
         patient_converter_requests_total {}\n",
        request_count
    );

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(body)
}
