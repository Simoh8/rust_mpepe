use anyhow::Result;

pub fn validate_phone_number(phone: &str) -> Result<String> {
    let cleaned = phone.trim().replace(" ", "");
    
    // Convert to M-Pesa format (254...)
    if cleaned.starts_with("07") && cleaned.len() == 10 {
        Ok(format!("254{}", &cleaned[1..]))
    } else if cleaned.starts_with("+254") && cleaned.len() == 13 {
        Ok(cleaned[1..].to_string())
    } else if cleaned.starts_with("254") && cleaned.len() == 12 {
        Ok(cleaned)
    } else {
        Err(anyhow::anyhow!("Invalid phone number format. Use: 07XXXXXXXX, +2547XXXXXXXX, or 2547XXXXXXXX"))
    }
}

pub fn validate_amount(amount: &str) -> Result<String> {
    let amount_num: f64 = amount.parse().map_err(|_| anyhow::anyhow!("Invalid amount"))?;
    
    if amount_num <= 0.0 {
        return Err(anyhow::anyhow!("Amount must be positive"));
    }
    
    // M-Pesa requires whole numbers for STK push
    if amount_num.fract() != 0.0 {
        return Err(anyhow::anyhow!("Amount must be a whole number"));
    }
    
    Ok(amount.to_string())
}