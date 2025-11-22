use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use reqwest::Client;
use std::env;
use crate::models::requests::StkPushRequest;
use crate::models::responses::{StkPushResponse, OAuthResponse};

pub struct MpesaService {
    client: Client,
    consumer_key: String,
    consumer_secret: String,
    shortcode: String,
    passkey: String,
    callback_url: String,
    stk_endpoint: String,
    oauth_url: String,
}

impl MpesaService {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();
        
        Ok(Self {
            client: Client::new(),
            consumer_key: env::var("MPESA_CONSUMER_KEY")?,
            consumer_secret: env::var("MPESA_CONSUMER_SECRET")?,
            shortcode: env::var("MPESA_SHORTCODE")?,
            passkey: env::var("MPESA_PASSKEY")?,
            callback_url: env::var("MPESA_CALLBACK_URL")?,
            stk_endpoint: env::var("MPESA_STK_ENDPOINT")
                .unwrap_or_else(|_| "https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest".into()),
            oauth_url: env::var("MPESA_OAUTH_URL")
                .unwrap_or_else(|_| "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials".into()),
        })
    }

    async fn get_oauth_token(&self) -> Result<String> {
        let auth = format!("{}:{}", self.consumer_key, self.consumer_secret);
        let auth_encoded = general_purpose::STANDARD.encode(auth.as_bytes());

        let res = self.client
            .get(&self.oauth_url)
            .header(reqwest::header::AUTHORIZATION, format!("Basic {}", auth_encoded))
            .send()
            .await?
            .error_for_status()?
            .json::<OAuthResponse>()
            .await?;

        Ok(res.access_token)
    }

    pub async fn initiate_stk_push(&self, request: StkPushRequest) -> Result<StkPushResponse> {
        let token = self.get_oauth_token().await?;
        
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let password_raw = format!("{}{}{}", self.shortcode, self.passkey, timestamp);
        let password = general_purpose::STANDARD.encode(password_raw.as_bytes());

        let body = serde_json::json!({
            "BusinessShortCode": self.shortcode,
            "Password": password,
            "Timestamp": timestamp,
            "TransactionType": "CustomerPayBillOnline",
            "Amount": request.amount,
            "PartyA": request.phone_number,
            "PartyB": self.shortcode,
            "PhoneNumber": request.phone_number,
            "CallBackURL": &self.callback_url,
            "AccountReference": request.account_reference,
            "TransactionDesc": request.transaction_desc,
        });

        let res = self.client
            .post(&self.stk_endpoint)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow!("STK push failed: HTTP {}: {}", status, text));
        }

        let response = res.json::<StkPushResponse>().await?;
        Ok(response)
    }
}