use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub id: i64,
    pub date: String,
    pub raw_commits: String,
    pub content: String,
    pub repo_ids: String,
    pub prompt_id: Option<i64>,
    pub created_at: String,
}
