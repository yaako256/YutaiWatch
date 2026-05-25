/*
shared/src/config/app_config.rs
app.tomlの内容を格納する構造体
*/
// 標準ライブラリ
// デシリアライズ用
use serde::Deserialize;
// ファイルパス用
use std::path::PathBuf;

/// 設定まとめ
#[derive(Debug, Deserialize)]
pub struct AppConfig {
  pub scraper: ScraperConfig,
  pub data: DataConfig,
  pub discord: DiscordConfig,
}

/// スクレイピング関連の設定
#[derive(Debug, Deserialize)]
pub struct ScraperConfig {
  pub dir_path: PathBuf,
  pub file_name: String,
  pub python_command: String,
}

/// データ関連の設定
#[derive(Debug, Deserialize)]
pub struct DataConfig {
  pub dir_path: PathBuf,
}

/// Discord関連の設定
#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
  pub notify_webhook: Vec<String>,
  pub error_webhook: Vec<String>,
}
