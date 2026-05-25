/*
infra/src/scraper.rs
pythonを実行する部分
*/
use std::process::Command;

use infra_config::ScraperConfig;
use shared::errors::{AppError, AppResult};

/// scraper pythonを実行する
pub fn run_scraper(config: &ScraperConfig) -> AppResult<ScraperOutput> {
  // → Command::new(python_command).arg(script_path).output()
  // → stdout を ScraperOutput に deserialize

  // scraper pythonを実行
  // その出力を取得
  let output = Command::new(config.python_command.clone())
    .arg(config.dir_path.join(config.file_name.clone()))
    .output()
    .expect("failed");

  Ok(ScraperOutput { _a: 0 })
}

// 仮で構造体を定義
pub struct ScraperOutput {
  _a: i8,
}
