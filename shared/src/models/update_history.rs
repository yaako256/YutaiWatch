/*
shared/src/models/update_history.rs
update_history.jsonlに保存するデータの構造体定義
*/
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHistory {
  pub detected_at: DateTime<FixedOffset>,
  pub ticker_symbol: String,
  pub ticker_name: String,
  pub published_at: String,
  pub title: String,
  pub url: String,
}
impl UpdateHistory {
  pub fn new() -> Self {
    Self {
      detected_at: Utc::now().fixed_offset(),
      ticker_symbol: String::new(),
      ticker_name: String::new(),
      published_at: String::new(),
      title: String::new(),
      url: String::new(),
    }
  }
}
