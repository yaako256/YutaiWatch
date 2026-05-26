/*
shared/src/errors.rs
エラー型の定義
*/
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("コマンドエラー: {0}")]
  InvalidCommand(String),
  #[error("設定エラー: {0}")]
  Config(String),
  #[error("スクレイプエラー: {0}")]
  Scrape(String),
  #[error("JSONパースエラー: {0}")]
  Parse(#[from] serde_json::Error),
  #[error("差分判定エラー: {0}")]
  Diff(String),
  #[error("Discord送信エラー: {0}")]
  Discord(String),
  #[error("ストレージエラー: {0}")]
  Storage(#[from] std::io::Error),
  #[error("Pythonプロセスエラー: {0}")]
  Process(String),
}

pub type AppResult<T> = Result<T, AppError>;
