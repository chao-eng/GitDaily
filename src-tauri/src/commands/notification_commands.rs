use crate::services::notification_service::NotificationService;
use crate::errors::Result;

#[tauri::command]
pub async fn test_feishu_notification(
    webhook_url: String,
) -> Result<()> {
    let title = "GitDaily 测试通知 🚀";
    let markdown_content = "这是一条来自 **GitDaily (日报之星)** 的测试推送消息。\n\n恭喜！飞书通知通道已成功打通并可正常运作！✨";
    NotificationService::send_feishu_markdown(&webhook_url, title, markdown_content).await
}
