use serde::{Deserialize, Serialize};

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
