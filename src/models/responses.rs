use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct OAuthResponse {
    pub access_token: String,
    pub expires_in: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StkPushResponse {
    pub MerchantRequestID: Option<String>,
    pub CheckoutRequestID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StkCallback {
    pub Body: CallbackBody,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallbackBody {
    pub stkCallback: StkCallbackData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StkCallbackData {
    pub MerchantRequestID: String,
    pub CheckoutRequestID: String,
    pub ResultCode: i32,
    pub ResultDesc: String,
    pub CallbackMetadata: Option<CallbackMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallbackMetadata {
    pub Item: Vec<CallbackItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallbackItem {
    pub Name: String,
    pub Value: serde_json::Value,
}