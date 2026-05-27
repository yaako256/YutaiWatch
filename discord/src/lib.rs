// まだ文字数制限処理などを付けていない！
// 送信エラー時もログを出すだけでリトライ処理などがない！
// 色も1色だけになってる！
// 後でつける！

// 送信用
use reqwest::blocking::Client;
// 指数バックオフ用(未実装)
// use std::thread::sleep;
// use std::time::Duration;

use logger::{log_error, log_warn};
use tracing::error;

use shared::ScrapedItem;
use shared::constants::discord;
use shared::errors::AppResult;

mod model;
use model::{DiscordEmbed, EmbedFooter, WebhookPayload};

// 複数WebhookへEmbedを送信する
pub fn send_notify(webhook_urls: &[String], items: &Vec<ScrapedItem>) -> AppResult<()> {
  // 全アイテムをEmbed化
  let embeds: Vec<DiscordEmbed> = items.iter().map(|item| build_embed(item)).collect();

  // ここに来てる時点でないと思うが
  // 空なら返す
  if embeds.is_empty() {
    logger::log(log_warn!("discord", "embedsが空"));
    return Ok(());
  }

  // 各WebhookURLに送信
  let client = Client::new();
  for url in webhook_urls {
    if let Err(e) = send_to_webhook(&client, &url.as_str(), &embeds) {
      logger::log(log_error!("discord", "Webhook送信失敗"));
      error!("Webhook送信失敗 (url: {}): {}", url, e);
    }
  }

  Ok(())
}

// embed形式で1件分の通知を組み立てる
fn build_embed(item: &ScrapedItem) -> DiscordEmbed {
  DiscordEmbed {
    // Embed上部タイトル
    // 銘柄名と証券コードを表示
    title: format!("{}({})", item.ticker_name, item.ticker_symbol),

    // 本文
    description: format!("タイトル：{}\nURL：{}", item.title, item.url),

    // タイトルリンク化しない
    url: None,

    // 空色 (Discord blurple寄り)
    color: Some(0x87CEEB),

    // 今回は未使用
    fields: vec![],

    // フッター
    footer: Some(EmbedFooter {
      text: format!("公開日：{}", item.published_at),
    }),

    // Discord右上timestampは使わない
    timestamp: None,
  }
}

// --- 単一Webhookへの送信（内部用）---
fn send_to_webhook(client: &Client, webhook_url: &str, embeds: &[DiscordEmbed]) -> AppResult<()> {
  // embedの最大送信数で分けて送信
  for chunk in embeds.chunks(discord::EMBED_COUNT_LIMIT) {
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
        let msg = format!(
          "Discord Webhook 送信失敗: HTTPステータス {}",
          response.status()
        );
        logger::log(log_error!("discord", msg));
        error!("{}", msg);
      }
      Err(e) => {
        let msg = format!("Discord Webhook 送信エラー: {}", e);
        logger::log(log_error!("discord", msg));
        error!("{}", msg);
      }
    }
  }

  Ok(())
}

/// ----------------------------------
/// ログ通知用（シンプルなtextメッセージ）
/// ----------------------------------
pub fn send_logs(webhook_urls: &[String]) -> AppResult<()> {
  // Discordの文字制限でチャンクに分割
  let chunks = logger::to_chunks(discord::CONTENT_LIMIT);

  if chunks.is_empty() {
    return Ok(());
  }

  let client = Client::new();
  for url in webhook_urls {
    for chunk in &chunks {
      if let Err(e) = send_text_to_webhook(&client, url.as_str(), chunk) {
        logger::log(log_error!("discord", "ログWebhook送信失敗"));
        error!("ログWebhook送信失敗 (url: {}): {}", url, e);
      }
    }
  }

  Ok(())
}

// --- 単一Webhookへのテキスト送信（ログ用）---
fn send_text_to_webhook(client: &Client, webhook_url: &str, content: &str) -> AppResult<()> {
  // コードブロックで囲んで見やすくする
  let formatted = format!("```\n{}\n```", content);

  let payload = WebhookPayload {
    content: Some(formatted),
    embeds: vec![],
  };

  let result = client.post(webhook_url).json(&payload).send();

  match result {
    Ok(response) if response.status().is_success() => Ok(()),
    Ok(response) => {
      let msg = format!(
        "Discord Webhook ログ送信失敗: HTTPステータス {}",
        response.status()
      );
      logger::log(log_error!("discord", msg));
      error!("{}", msg);
      Ok(())
    }
    Err(e) => {
      let msg = format!("Discord Webhook ログ送信エラー: {}", e);
      logger::log(log_error!("discord", msg));
      error!("{}", msg);
      Ok(())
    }
  }
}
