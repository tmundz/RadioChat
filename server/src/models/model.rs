use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub uid: Uuid,
    pub user_name: String,
    pub password: String, // need to set up a hash
    #[serde(rename = "createAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_admin: bool,
}
