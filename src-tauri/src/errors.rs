use thiserror::Error;
use serde::{Serialize, Serializer};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("仓库错误: {0}")]
    RepoError(String),

    #[error("Git 操作失败: {0}")]
    GitError(String),

    #[error("AI API 调用失败: {0}")]
    AiError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rusqlite::Error),

    #[error("网络请求失败: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("序列化错误: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
