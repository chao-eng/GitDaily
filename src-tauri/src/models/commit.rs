use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitLogQuery {
    pub repo_ids: Vec<i64>,
    pub date_from: String,
    pub date_to: String,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitRecord {
    pub hash: String,
    pub short_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64,
    pub datetime: String,
    pub message: String,
    pub repo_name: String,
    pub diff_stat: Option<DiffStat>,
    pub is_merge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffStat {
    pub files_changed: Vec<String>,
    pub insertions: u32,
    pub deletions: u32,
}
