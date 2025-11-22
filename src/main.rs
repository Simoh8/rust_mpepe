mod routes;
mod services;
mod models;

use actix_web::{web, App, HttpServer};
use routes::{stk, callback};
use services::mpesa::MpesaService;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mpesa_service = match MpesaService::new() {
        Ok(service) => {
            web::Data::new(service)
        }
        Err(e) => {
            std::process::exit(1);
        }
    };

    let callback_state = web::Data::new(callback::CallbackState::default());


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