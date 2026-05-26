// HashID生成用
use sha2::{Digest, Sha256};

use shared::State;
use shared::errors::AppResult;
use shared::{ScrapedItem, ScraperOutput};
use tracing::info;

// デバッグ用関数
pub fn debug() {
  println!("Hello from shared!");
}

// 差分判定の本体
// stateと照合し、未通知のitemだけを返す
pub fn detect_new_item(output: &ScraperOutput, state: &State) -> AppResult<Vec<ScrapedItem>> {
  info!("今から差分判定するよ");

  // デバッグ用に直値で返す
  Ok(
    (vec![ScrapedItem {
      item_key: "20260525_002".to_string(),
      ticker_symbol: "5678".to_string(),
      ticker_name: "テストホールディングス".to_string(),
      published_at: "2026-05-25T10:30:00+09:00".to_string(),
      title: "優待新設のお知らせ".to_string(),
      url: "https://example.com/item2".to_string(),
    }]),
  )
}

pub fn generate_fingerprint(item: &ScrapedItem) -> String {
  let source = format!(
    "{}|{}|{}|{}",
    normalize_url(&item.url),
    normalize_datetime(&item.published_at),
    item.ticker_symbol.trim(),
    normalize_title(&item.title),
  );

  let hash = Sha256::digest(source.as_bytes());

  format!("sha256:{:x}", hash)
}
