use serde::{Deserialize, Serialize};
use worker::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug, Serialize, Deserialize)]
pub enum BillingStatus {
    Free,
    Pro,
    PastDue,
}

impl ToString for BillingStatus {
    fn to_string(&self) -> String {
        match self {
            BillingStatus::Free => "free".to_string(),
            BillingStatus::Pro => "pro".to_string(),
            BillingStatus::PastDue => "past_due".to_string(),
        }
    }
}

pub struct Razorpay {
    pub webhook_secret: String,
}

impl Razorpay {
    pub fn new(secret: String) -> Self {
        Self { webhook_secret: secret }
    }

    pub fn verify_signature(&self, body: &str, signature: &str) -> bool {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self.webhook_secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(body.as_bytes());
        
        let expected = hex::encode(mac.finalize().into_bytes());
        expected == signature
    }
}

pub mod db {
    use super::*;
    use worker::d1::D1Database;

    pub async fn grant_credits(db: &D1Database, tenant_id: &str, credits: i64) -> Result<()> {
        db.prepare("UPDATE tenants SET prepaid_credits = prepaid_credits + ? WHERE id = ?")
            .bind(&[credits.into(), tenant_id.into()])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn reset_monthly_quota(db: &D1Database, subscription_id: &str, next_period_end_unix: i64) -> Result<()> {
        db.prepare("UPDATE tenants SET monthly_free_used = 0, current_period_end = datetime(?, 'unixepoch'), billing_status = 'pro' WHERE subscription_id = ?")
            .bind(&[next_period_end_unix.into(), subscription_id.into()])?
            .run()
            .await?;
        Ok(())
    }
}

/// Calculate credits based on amount and unit cost.
/// e.g. amount_cents=100 (1 USD), cost_per_unit=0.035 cents -> ~2857 credits.
pub fn calculate_credits(amount_cents: i64, cost_per_unit_cents: f64) -> i64 {
    (amount_cents as f64 / cost_per_unit_cents).floor() as i64
}
