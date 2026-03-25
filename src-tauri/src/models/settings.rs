use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConfig {
    pub api_base_url: String,
    pub api_key: String,
    pub model_name: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://api.openai.com/v1".to_string(),
            api_key: "".to_string(),
            model_name: "gpt-4o-mini".to_string(),
            temperature: 0.5,
            max_tokens: 2048,
        }
    }
}
