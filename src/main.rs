mod routes;
mod services;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse};
use routes::{stk, callback};
use services::mpesa::MpesaService;
use std::io;

// Embedded static files
const INDEX_HTML: &str = include_str!("../public/index.html");
const STYLES_CSS: &str = include_str!("../public/styles.css");
const MAIN_JS: &str = include_str!("../public/main.js");

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize M-Pesa service
    let mpesa_service = match MpesaService::new() {
        Ok(service) => {
            println!("M-Pesa service initialized successfully");
            web::Data::new(service)
        }
        Err(e) => {
            eprintln!("Failed to initialize M-Pesa service: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize callback state
    let callback_state = web::Data::new(callback::CallbackState::default());

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(mpesa_service.clone())
            .app_data(callback_state.clone())
            // Serve embedded static files
            .route("/", web::get().to(serve_index))
            .route("/styles.css", web::get().to(serve_css))
            .route("/main.js", web::get().to(serve_js))
            // API routes
            .route("/stk-push", web::post().to(stk::initiate_stk_push))
            .route("/callback", web::post().to(callback::handle_callback))
            .route("/last-callback", web::get().to(callback::get_last_callback))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn serve_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX_HTML)
}

async fn serve_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(STYLES_CSS)
}

async fn serve_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(MAIN_JS)
}