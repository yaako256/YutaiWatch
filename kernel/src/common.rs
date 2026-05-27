/*
kernel/src/common.rs
kernelの共通処理を定義
*/
// 標準クレート
// ジッター待機用
use std::thread::sleep;
use std::time::Duration;

// 外部クレート
// ジッター用乱数
use rand::Rng;
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

/// ジッター処理
/// 指定範囲で一様ジッターしてsleepする
pub fn jitter_sleep(min_secs: u64, max_secs: u64) {
  // 乱数をインスタンス
  let mut rng = rand::thread_rng();

  // ジッター秒数を決める
  let jitter_secs: u64 = rng.gen_range(min_secs..=max_secs);

  info!(
    "ジッター開始: {} 秒スリープします (range: {}..={})",
    jitter_secs, min_secs, max_secs
  );

  // 待機
  sleep(Duration::from_secs(jitter_secs));

  info!(
    "ジッター終了: {} 秒スリープしました (range: {}..={})",
    jitter_secs, min_secs, max_secs
  );
}
