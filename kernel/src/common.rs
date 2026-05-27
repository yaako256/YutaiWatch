/*
kernel/src/common.rs
kernelの共通処理を定義
*/
// 外部クレート
// ログ出力用
use tracing::{error, info};

// Workspace内クレート
use infra_config::AppConfig;
use shared::errors::AppResult;

/// 終了時に実行する処理
pub fn finish(config: &AppConfig) -> AppResult<()> {
  // loggerをdiscord送信
  if let Err(e) = discord::send_logs(&config.discord.logs_webhook) {
    error!("ログの送信に失敗");
    return Err(e);
  }

  info!("終了処理を実行しました");

  Ok(())
}
