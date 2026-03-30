use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Workdays,
}

impl Default for ScheduleFrequency {
    fn default() -> Self {
        Self::Daily
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchedulerConfig {
    pub enabled: bool,
    pub frequency: ScheduleFrequency,
    pub hour: u8,
    pub minute: u8,
    pub day_of_week: Option<u8>, // 0-6, Sunday-Saturday
    pub repo_ids: Vec<i64>,
    pub prompt_id: Option<i64>,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            frequency: ScheduleFrequency::Daily,
            hour: 18,
            minute: 30,
            day_of_week: Some(5), // 默认周五生成周报
            repo_ids: Vec::new(),
            prompt_id: None,
        }
    }
}
