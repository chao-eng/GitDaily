use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub is_builtin: bool,
    pub is_default: bool,
    pub created_at: String,
}
