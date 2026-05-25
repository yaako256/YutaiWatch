/*
shared/src/models/update_history.rs
update_history.jsonlに保存するデータの構造体定義
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHistory {
  pub detected_at: String, // 時間関連の奴に変える
  pub ticker_symbol: String,
  pub ticker_name: String,
  pub published_at: String,
  pub title: String,
  pub url: String,
}
