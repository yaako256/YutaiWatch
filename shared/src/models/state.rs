/*
shared/src/models/state.rs
state.jsonfに保存するデータの構造体定義
*/
use super::scraper_output::ScrapedItem;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
  // pub schema_version: u32,
  pub creat_state_at: DateTime<FixedOffset>,
  pub last_update_at: DateTime<FixedOffset>,
  // pub last_snapshot_hash: String,
  pub last_seen_item: ScrapedItem,
  pub notified_item_keys: Vec<String>,
}
