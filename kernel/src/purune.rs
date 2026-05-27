/*
kernel/src/purune.rs
purune処理の関数を定義
*/
// Workspace内クレート
use infra_config::AppConfig;
use logger::{self, log_error, log_info};
use shared::errors::AppResult;

// 自クレート
use super::common::finish;

/// prune実行関数
pub fn run_prune(config: &AppConfig) -> AppResult<()> {
  // 調整することをdiscordに送信
  logger::log(log_info!("prune", "prune処理実行開始"));

  // detect_history.jsonlの調整
  if let Err(e) = infra::storage::prune_detect_history(config.data.dir_path.as_path()) {
    logger::log(log_error!("prune", "detect_historyの圧縮失敗"));
    return Err(e);
  }
  // update_history.jsonlの調整
  if let Err(e) = infra::storage::prune_update_history(config.data.dir_path.as_path()) {
    logger::log(log_error!("prune", "update_historyの圧縮失敗"));
    return Err(e);
  }

  // 調整したことをdiscordに送信
  logger::log(log_info!("prune", "prune処理実行完了"));

  // 終了処理
  finish(config)?;

  Ok(())
}
