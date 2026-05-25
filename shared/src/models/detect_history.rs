/*
shared/src/models/detect_history.rs
detect_history.jsonlに保存するデータの構造体定義
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectHistory {
  pub detected_at: String,
  pub updated: bool,
}
