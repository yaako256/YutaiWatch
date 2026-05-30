/*
shared/src/models/detect_history.rs
detect_history.jsonlに保存するデータの構造体定義
*/
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectHistory {
  pub detected_at: DateTime<FixedOffset>,
  pub new_items: usize,
}
