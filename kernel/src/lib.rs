use infra_config::AppConfig;
use shared::ScrapedItem;
use shared::errors::AppResult;

// 外部ライブラリ
// ログ出力
use tracing::info;

/// デバッグ用関数
pub fn debug() {
  println!("Hello from kernel!");
}

/// monitor実行関数
pub fn run_monitor(config: &AppConfig) -> AppResult<()> {
  // デバッグ用
  shared::debug();
  monitor::debug();
  discord::debug();

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

  // discordに送信
  discord::send_notify(config.discord.notify_webhook.clone(), items)?;

  // stateデータ入力テスト
  infra::storage::save_state(config.data.dir_path.as_path(), &state)?;

  // detect_historyデータ入力テスト
  infra::storage::append_detect_history(
    config.data.dir_path.as_path(),
    &shared::DetectHistory {
      detected_at: chrono::Utc::now().into(),
      updated: true,
    },
  )?;

  // update_history
  infra::storage::append_update_history(
    config.data.dir_path.as_path(),
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
