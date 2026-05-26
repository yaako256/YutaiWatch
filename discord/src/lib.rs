// まだ文字数制限処理などを付けていない！
// 送信エラー時もログを出すだけでリトライ処理などがない！
// 色も1色だけになってる！
// 後でつける！

// 送信用
use reqwest::blocking::Client;
// 指数バックオフ用
use std::thread::sleep;
use std::time::Duration;

use tracing::error;

use shared::ScrapedItem;
use shared::errors::AppResult;

mod model;
use model::{DiscordEmbed, EmbedField, EmbedFooter, WebhookPayload};

// デバッグ用関数
pub fn debug() {
  println!("hello discord");
}

// embed形式で1件分の通知を組み立てる
pub fn build_embed(item: &ScrapedItem) -> DiscordEmbed {
  // 内容
  let description = format!(
    concat!(
      "銘柄名    ：{}\n",
      "証券コード :{}\n",
      "タイトル  ：{}\n",
      "URL      ：<{}>"
    ),
    item.ticker_name, item.ticker_symbol, item.title, item.url,
  );

  DiscordEmbed {
    // Embed上部タイトル
    title: "適時開示検出".to_string(),

    // 本文
    description,

    // タイトルリンク化しない
    url: None,

    // 空色 (Discord blurple寄り)
    color: Some(0x87CEEB),

    // 今回は未使用
    fields: vec![],

    // フッター
    footer: Some(EmbedFooter {
      text: format!("検出日時：{}", item.published_at),
    }),

    // Discord右上timestampは使わない
    timestamp: None,
  }
}

// 複数WebhookへEmbedを送信する
pub fn send_notify(webhook_urls: Vec<String>, items: Vec<ScrapedItem>) -> AppResult<()> {
  // 全アイテムをEmbed化
  let embeds: Vec<DiscordEmbed> = items.iter().map(|item| build_embed(item)).collect();

  // ここに来てる時点でないと思うが
  // 空なら返す
  if embeds.is_empty() {
    return Ok(());
  }

  // 各WebhookURLに送信
  let client = Client::new();
  for url in webhook_urls {
    if let Err(e) = send_to_webhook(&client, &url.as_str(), embeds.clone()) {
      error!("Webhook送信失敗 (url: {}): {}", url, e);
    }
  }

  Ok(())
}

// --- 単一Webhookへの送信（内部用）---
fn send_to_webhook(client: &Client, webhook_url: &str, embeds: Vec<DiscordEmbed>) -> AppResult<()> {
  const MAX_EMBEDS_PER_REQUEST: usize = 10;

  // embedの最大送信数で分けて送信
  for chunk in embeds.chunks(MAX_EMBEDS_PER_REQUEST) {
    let payload = WebhookPayload {
      content: None,
      embeds: chunk.to_vec(),
    };

    // 実際に送る
    let result = client.post(webhook_url).json(&payload).send();

    match result {
      Ok(response) if response.status().is_success() => {
        // 送信成功
      }
      Ok(response) => {
        error!(
          "Discord Webhook 送信失敗: HTTPステータス {}",
          response.status()
        );
      }
      Err(e) => {
        error!("Discord Webhook 送信エラー: {}", e);
      }
    }
  }

  Ok(())
}

// エラー通知用（シンプルなtextメッセージ）
pub fn send_error(webhook_urls: Vec<&str>, message: &str) -> AppResult<()> {
  Ok(())
}
