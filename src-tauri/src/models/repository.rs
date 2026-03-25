use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub is_active: bool,
    pub created_at: String,
}
