/*
kernel/src/monitor.rs
monitor処理の関数を定義
*/
// Workspace内クレート
use infra_config::AppConfig;
use logger::{self, log_error, log_info};
use shared::{
  ScrapedItem, ScraperOutput, State, UpdateHistory,
  constants::file::NOTIFIED_KEYS_LIMIT,
  errors::{AppError, AppResult},
};

// 自クレート
use super::common::finish;
use super::initialize::run_initialize;

/// monitor実行関数
pub fn run_monitor(config: &AppConfig) -> AppResult<()> {
  // ----------------------
  // stateデータ取得(なければ初回フローの実行)
  // ----------------------
  // stateをロード
  let state: AppResult<Option<State>> = infra::storage::load_state(&config.data.dir_path.as_path());
  // 取得に成功したか
  // 取得に失敗したら初回フロー
  let state: Option<State> = match state {
    Ok(s) => s,
    Err(e) => {
      logger::log(log_error!("monitor", "Stateの取得に失敗"));
      logger::log(log_info!("monitor", "initialize処理を実行"));

      // 初回フローの実行
      if let Err(e) = run_initialize(config) {
        logger::log(log_error!("monitor", "initialize処理失敗"));
        return Err(e);
      }

      // Err返し終了
      return Err(AppError::Process(format!(
        "Stateの取得に失敗。initフローの実行をしました: {}",
        e
      )));
    }
  };

  // stateに中身があったか
  // なかったら初回フロー
  let mut state: State = match state {
    Some(s) => s,
    None => {
      logger::log(log_error!("monitor", "Stateに値が存在しません"));
      logger::log(log_info!("monitor", "initialize処理を実行"));

      // 初回フローの実行
      if let Err(e) = run_initialize(config) {
        logger::log(log_error!("monitor", "initialize処理失敗"));
        return Err(e);
      }

      // Err返し終了
      return Err(AppError::Process(
        "Stateに中身がなかったためinitフローを実行しました".to_string(),
      ));
    }
  };

  // ----------------------
  // 監視対象サイトをスクレイピング
  // pythonを実行
  // ----------------------
  // スクレイピング部分実行
  let output: ScraperOutput = match infra::scraper::run_scraper(&config.scraper) {
    Ok(o) => o,
    Err(e) => {
      logger::log(log_error!("monitor", "スクレイピング失敗"));

      // 終了処理
      finish(config)?;

      // Err返し終了
      return Err(AppError::Process(format!("{}", e)));
    }
  };

  // ----------------------
  // 更新差分判定
  // 更新なし → 終了処理
  // 更新あり → 処理実行
  // ----------------------
  let new_data: Vec<(String, ScrapedItem)> = match monitor::detect_new_item(&output, &state) {
    Ok(d) => d,
    Err(e) => {
      logger::log(log_error!("monitor", "差分判定失敗"));
      return Err(e);
    }
  };

  // 空だったらdetect_history更新だけして実行終了
  if new_data.is_empty() {
    // detect_historyの更新
    if let Err(e) = infra::storage::append_detect_history(
      config.data.dir_path.as_path(),
      &shared::DetectHistory {
        detected_at: output.fetched_at,
        updated: false,
      },
    ) {
      logger::log(log_error!("monitor", "detect_historyの更新失敗"));
      return Err(e);
    }

    // 終了処理
    finish(config)?;

    // Okを返し終了
    return Ok(());
  }

  // タプルベクトルの戻り値を各ベクトルに分解
  let (fingerprints, scraped_items): (Vec<String>, Vec<ScrapedItem>) = new_data.into_iter().unzip();

  // ----------------------
  // 更新内容を通知
  // ----------------------
  // discordに送信
  if let Err(e) = discord::send_notify(&config.discord.notify_webhook, &scraped_items) {
    logger::log(log_error!("monitor", "discordに送信失敗"));
    return Err(e);
  }

  // ----------------------
  // state更新
  // ----------------------
  // stateに追加
  // 最終更新時間を保存
  state.last_update_at = output.fetched_at;
  // 最後のitemを保存(上でscraped_itemsが空じゃないことは保証しているのでunwrap())
  state.last_seen_item = scraped_items.first().cloned().unwrap();
  // notified_item_keysを保存
  state.notified_item_keys.extend(fingerprints);
  // notified_item_keysが一定以上だったら古いものから消す
  if state.notified_item_keys.len() > NOTIFIED_KEYS_LIMIT {
    // オーバー分を計算
    let overflow = state.notified_item_keys.len() - NOTIFIED_KEYS_LIMIT;
    // 古い部分を削除
    state.notified_item_keys.drain(0..overflow);
  }
  // state.jsonを更新
  if let Err(e) = infra::storage::save_state(config.data.dir_path.as_path(), &state) {
    logger::log(log_error!("monitor", "state.json更新失敗"));
    return Err(e);
  }

  // ----------------------
  // detect_history更新
  // ----------------------
  if let Err(e) = infra::storage::append_detect_history(
    config.data.dir_path.as_path(),
    &shared::DetectHistory {
      detected_at: output.fetched_at,
      updated: true,
    },
  ) {
    logger::log(log_error!("monitor", "detect_history.json更新失敗"));
    return Err(e);
  }

  // ----------------------
  // update_history更新
  // ----------------------
  for item in scraped_items {
    if let Err(e) = infra::storage::append_update_history(
      config.data.dir_path.as_path(),
      &UpdateHistory {
        detected_at: output.fetched_at,
        ticker_symbol: item.ticker_symbol,
        ticker_name: item.ticker_name,
        published_at: item.published_at,
        title: item.title,
        url: item.url,
      },
    ) {
      logger::log(log_error!("monitor", "update_history更新失敗"));
      return Err(e);
    }
  }

  // 終了処理
  finish(config)?;

  Ok(())
}
