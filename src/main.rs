mod routes;
mod services;
mod models;

use actix_web::{web, App, HttpServer};
use routes::{stk, callback};
use services::mpesa::MpesaService;
use std::io;

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

    println!("Starting server at http://localhost:8080");
    println!("Open http://localhost:8080 in your browser to test STK push");

    HttpServer::new(move || {
        App::new()
            .app_data(mpesa_service.clone())
            .app_data(callback_state.clone())
            .service(web::resource("/").to(serve_index))
            .route("/stk-push", web::post().to(stk::initiate_stk_push))
            .route("/callback", web::post().to(callback::handle_callback))
            .route("/last-callback", web::get().to(callback::get_last_callback))
            .service(actix_files::Files::new("/static", "./public"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn serve_index() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./public/index.html")?)
}