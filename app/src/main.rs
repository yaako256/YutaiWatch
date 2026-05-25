// 標準ライブラリ
use std::fs::File;

use infra;
use infra_config;
use shared;

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

  // 設定読み込み
  let config = infra_config::load_config()?;
  info!("{:#?}", config);

  // スクレイピング部分実行のテスト
  infra::scraper::run_scraper(&config.scraper)?;

  Ok(())
}
