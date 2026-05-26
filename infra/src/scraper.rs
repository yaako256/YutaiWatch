/*
infra/src/scraper.rs
pythonを実行する部分
*/
use std::process::Command;

use serde::Deserialize;
use tracing::info;

use infra_config::ScraperConfig;
use shared::ScraperOutput;
use shared::errors::{AppError, AppResult};

/// scraper pythonを実行する
/// ScraperOutput に deserializeして戻す
pub fn run_scraper(config: &ScraperConfig) -> AppResult<ScraperOutput> {
  // この関数のやることメモ
  // → Command::new(python_command).arg(script_path).output()
  // → stdout を ScraperOutput に deserialize

  // scraper pythonを実行
  // その出力を取得
  let output = Command::new(config.python_command.clone())
    .arg(config.dir_path.join(config.file_name.clone()))
    .output()
    .map_err(|e| AppError::Process(format!("Python起動失敗: {}", e)))?;

  // 実行が失敗してたらエラーを返す
  // 上のmap_errはOSレベルのエラー(コマンドが存在しない等)しか返さないため
  if !output.status.success() {
    return Err(AppError::Process(format!(
      "Python実行失敗: {}",
      String::from_utf8_lossy(&output.stderr)
    )));
  }

  // stdout を文字列(utf-8)化
  let stdout = String::from_utf8_lossy(&output.stdout);

  // JSON deserialize
  let result: ScraperOutput = serde_json::from_str(&stdout)?;

  // デバッグ用ログ
  info!("python stdout: {:#?}", result);

  Ok(result)
}
