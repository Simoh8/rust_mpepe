use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StkPushRequest {
    pub amount: String,
    pub phone_number: String,
    pub account_reference: String,
    pub transaction_desc: String,
}