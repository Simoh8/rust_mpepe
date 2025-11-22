use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::services::mpesa::MpesaService;

#[derive(Deserialize)]
pub struct StkPushForm {
    pub phone: String,
    pub amount: String,
}

pub async fn initiate_stk_push(
    form: web::Json<StkPushForm>,
    mpesa_service: web::Data<MpesaService>,
) -> HttpResponse {

    // Validate inputs
    if form.phone.is_empty() || form.amount.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Phone number and amount are required"
        }));
    }

    let request = crate::models::requests::StkPushRequest {
        amount: form.amount.clone(),
        phone_number: form.phone.clone(),
        account_reference: "WEB_PAYMENT".to_string(),
        transaction_desc: "Online Payment".to_string(),
    };

    match mpesa_service.initiate_stk_push(request).await {
        Ok(response) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "STK push initiated successfully",
                "data": response
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to initiate STK push: {}", e)
            }))
        }
    }
}