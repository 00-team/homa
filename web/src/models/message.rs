use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Message {
    pub id: i64,
    pub user: i64,
    pub activation_id: i64,
    pub timestamp: i64,
    pub text: String,
    pub code: String,
    pub country: String,
    pub service: String,
    pub received_at: String,
    pub seen: bool,
}
