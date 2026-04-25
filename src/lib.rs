use serde::{Deserialize, Serialize};
use worker::*;

pub mod auth;
pub mod billing;
pub mod tenant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub billing_status: String,
}
