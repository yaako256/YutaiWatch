/*
shared/src/models/state.rs
state.jsonfに保存するデータの構造体定義
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
  pub schema_version: u32,
  pub last_success_at: String,
  pub last_snapshot_hash: String,
  pub last_seen_item_key: String,
  pub notified_item_keys: Vec<String>,
}
