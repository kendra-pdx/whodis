use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::UtcDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Verification<T> {
    Unverified(T),
    Verified(T, UtcDateTime),
}

pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub email_address: Verification<String>,
    pub mobile_phone: Option<Verification<String>>,
}
