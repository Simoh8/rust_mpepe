use actix_web::{web, HttpResponse, Result};
use std::sync::Mutex;
use crate::models::responses::StkCallback;

#[derive(Default)]
pub struct CallbackState {
    pub last_callback: Mutex<Option<StkCallback>>,
}

pub async fn handle_callback(
    callback: web::Json<StkCallback>,
    state: web::Data<CallbackState>,
) -> Result<HttpResponse> {
    println!("Received callback: {:#?}", callback.0);
    
    // Store the callback for later retrieval
    let mut last_callback = state.last_callback.lock().unwrap();
    *last_callback = Some(callback.into_inner());
    
    Ok(HttpResponse::Ok().body("Callback received successfully"))
}

pub async fn get_last_callback(
    state: web::Data<CallbackState>,
) -> Result<HttpResponse> {
    let last_callback = state.last_callback.lock().unwrap();
    
    match &*last_callback {
        Some(callback) => Ok(HttpResponse::Ok().json(callback)),
        None => Ok(HttpResponse::NotFound().body("No callback received yet")),
    }
}