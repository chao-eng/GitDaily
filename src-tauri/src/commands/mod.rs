pub mod repo_commands;
pub mod git_commands;
pub mod ai_commands;
pub mod prompt_commands;
pub mod report_commands;
pub mod settings_commands;
pub mod scheduler_commands;

use serde::Serialize;

#[derive(Serialize)]
pub struct TestResult {
    pub success: bool,
    pub message: String,
}
