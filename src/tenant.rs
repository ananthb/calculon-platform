use serde::{Deserialize, Serialize};
use worker::*;

pub async fn get_user_tenants(db: &D1Database, user_id: &str) -> Result<Vec<TenantInfo>> {
    let rows = db.prepare("SELECT t.id, t.name, t.billing_status, tu.role FROM tenants t JOIN tenant_users tu ON t.id = tu.tenant_id WHERE tu.user_id = ?")
        .bind(&[user_id.into()])?
        .all()
        .await?
        .results::<TenantInfo>()?;
    Ok(rows)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantInfo {
    pub id: String,
    pub name: String,
    pub billing_status: String,
    pub role: String,
}
