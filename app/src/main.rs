// 標準ライブラリ
use std::fs::File;

use discord;
use infra;
use infra_config;
use monitor;
use shared::{self, ScrapedItem, errors::AppError};

// 外部ライブラリ
// ログ出力
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // ログ出力の設定
  // 環境変数 RUST_LOG からログレベルを読み込み、無ければデフォルトで「info」にする
  // ファイル出力とかをすぐ増やせて拡張性ましまし、最近流行の定義方法らしい
  tracing_subscriber::registry()
    // ターミナルログ出力定義。
    .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
    // ログファイル出力定義。
    .with(
      fmt::layer()
        .with_writer(File::create("app.log").expect("failed to create log file"))
        .with_ansi(false),
    )
    // 標準出力の設定。
    .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
    .init();

  // デバッグ用
  println!("Hello, world!");
  shared::debug();
  infra_config::debug();
  monitor::debug();
  discord::debug();

  // 設定読み込み
  let config = infra_config::load_config()?;
  // info!("{:#?}", config);

  // スクレイピング部分実行のテスト
  let output = match infra::scraper::run_scraper(&config.scraper) {
    Ok(o) => o,
    Err(e) => {
      info!("{:#?}", e);
      return Ok(());
    }
  };

  // stateデータ取得のテスト
  let state = infra::storage::load_state(&config.data.dir_path.as_path());
  info!("{:#?}", state);

  let state = state?;

  let state = match state {
    Some(s) => s,
    None => {
      println!("値なし");
      return Ok(());
    }
  };

  // 差分検出のデバッグ
  let new_data: Vec<(String, ScrapedItem)> = monitor::detect_new_item(&output, &state)?;

  // ScrapedItemだけ取り出す
  let items: Vec<ScrapedItem> = new_data.into_iter().map(|(_, item)| item).collect();

  discord::send_notify(config.discord.notify_webhook, items)?;

  // stateデータ入力テスト
  infra::storage::save_state(&config.data.dir_path.as_path(), &state)?;

  // detect_historyデータ入力テスト
  infra::storage::append_detect_history(
    &config.data.dir_path.as_path(),
    &shared::DetectHistory {
      detected_at: chrono::Utc::now().into(),
      updated: true,
    },
  )?;

  // update_history
  infra::storage::append_update_history(
    &config.data.dir_path.as_path(),
    &shared::UpdateHistory {
      detected_at: chrono::Utc::now().into(),
      ticker_symbol: "asdgf".to_string(),
      ticker_name: "asdga".to_string(),
      published_at: chrono::Utc::now().into(),
      title: "gfds".to_string(),
      url: "sdhd".to_string(),
    },
  )?;
  Ok(())
}
