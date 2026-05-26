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

/// 差分判定の本体
/// stateと照合し、未通知のitemだけを返す
pub fn detect_new_item(output: &ScraperOutput, state: &State) -> AppResult<Vec<ScrapedItem>> {
  info!("今から差分判定するよ");

  // デバッグ用に直値で返す
  Ok(
    (vec![ScrapedItem {
      ticker_symbol: "5678".to_string(),
      ticker_name: "テストホールディングス".to_string(),
      published_at: "2026-05-25T10:30:00+09:00".to_string(),
      title: "優待新設のお知らせ".to_string(),
      url: "https://example.com/item2".to_string(),
    }]),
  )
}

/// 判別用のfingerprintを出力する関数
pub fn generate_fingerprint(item: &ScrapedItem) -> String {
  // 内容をつなげたベース文字列を作成
  let source = format!(
    "{}|{}|{}|{}",
    &item.url, &item.published_at, &item.ticker_symbol, &item.title,
  );
  // Hash化
  let hash = Sha256::digest(source.as_bytes());

  // stringとしてreturn
  format!("sha256:{:x}", hash)
}
