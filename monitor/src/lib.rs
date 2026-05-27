// HashID生成用
use sha2::{Digest, Sha256};

use logger::log_warn;
use shared::State;
use shared::errors::AppResult;
use shared::{ScrapedItem, ScraperOutput};
use tracing::info;

/// 差分判定の本体
/// stateと照合し、未通知のitemだけを返す
pub fn detect_new_item(
  output: &ScraperOutput,
  state: &State,
) -> AppResult<Vec<(String, ScrapedItem)>> {
  // デバッグ用ログ
  info!("今から差分判定するよ");

  // ScraperOutputからitemを取得
  let items = &output.items;

  // itemが空の場合の処理
  if items.is_empty() {
    info!("取得したitemが空でした");
    logger::log(log_warn!("detect_new_item", "ScraperOutput.itemsが空"));
    return Ok(vec![]);
  }

  // 新しいitemを入れるところ
  let mut new_items: Vec<(String, ScrapedItem)> = Vec::new();

  // itemをforで回す
  for item in items {
    // Sha256IDの生成
    let fingerprint = generate_fingerprint(item);

    if !state.notified_item_keys.contains(&fingerprint) {
      info!("新着item検出: sha256:{}", fingerprint);
      new_items.push((fingerprint, item.clone())); // タプルで追加
    } else {
      info!("既通知のitemをスキップ: sha256:{}", fingerprint);
    }
  }

  // デバッグ用に直値で返す
  Ok(new_items)
}

/// 判別用のfingerprintを出力する関数
fn generate_fingerprint(item: &ScrapedItem) -> String {
  // 内容をつなげたベース文字列を作成
  let source = format!(
    "{}|{}|{}|{}",
    &item.url, &item.published_at, &item.ticker_symbol, &item.title,
  );
  // Hash化
  let hash = Sha256::digest(source.as_bytes());

  // stringとしてreturn
  format!("{:x}", hash)
}
