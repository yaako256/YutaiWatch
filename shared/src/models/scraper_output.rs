use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

// 仮で構造体を定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperOutput {
  // これもいらないが、拡張性が持たせれる
  // Jsonの仕様変更をしたとき、Rust側でスキーマを分ける用
  // pub schema_version: u32,
  // どこから取得したか。1サイト監視なのでいらない
  // pub source: String,
  // 取得時間
  pub fetched_at: DateTime<FixedOffset>,
  // 内容物
  pub items: Vec<ScrapedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedItem {
  pub ticker_symbol: String,
  pub ticker_name: String,
  pub published_at: String,
  pub title: String,
  pub url: String,
}
