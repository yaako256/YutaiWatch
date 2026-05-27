/*
infra/src/scraper.rs
pythonを実行する部分
*/
use std::process::Command;

// use tracing::info;

use infra_config::ScraperConfig;
use logger::log_error;
use shared::ScraperOutput;
use shared::errors::{AppError, AppResult};

/// scraper pythonを実行する
/// ScraperOutput に deserializeして戻す
pub fn run_scraper(config: &ScraperConfig) -> AppResult<ScraperOutput> {
  // scraper pythonを実行
  // その出力を取得
  let output = match Command::new(config.python_command.clone())
    .arg(config.dir_path.join(config.file_name.clone()))
    .output()
  {
    Ok(o) => o,
    Err(e) => {
      logger::log(log_error!("run_scraper", "Python起動失敗"));
      // Err返し終了
      return Err(AppError::Process(format!("Python起動失敗: {}", e)));
    }
  };

  // 実行が失敗してたらエラーを返す
  // 上のmap_errはOSレベルのエラー(コマンドが存在しない等)しか返さないため
  if !output.status.success() {
    logger::log(log_error!("run_scraper", "Python処理失敗"));
    return Err(AppError::Process(format!(
      "Python実行失敗: {}",
      String::from_utf8_lossy(&output.stderr)
    )));
  }

  // stdout を文字列(utf-8)化
  let stdout = String::from_utf8_lossy(&output.stdout);

  // JSON deserialize
  let result: ScraperOutput = match serde_json::from_str(&stdout) {
    Ok(r) => r,
    Err(e) => {
      logger::log(log_error!("run_scraper", "deserialize失敗"));
      // Err返し終了
      return Err(AppError::Process(format!("deserialize失敗: {}", e)));
    }
  };

  // デバッグ用ログ
  // info!("python stdout: {:#?}", result);

  Ok(result)
}
