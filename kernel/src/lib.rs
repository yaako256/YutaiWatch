use std::fs;

use infra_config::AppConfig;
use logger::{self, log_error, log_info};
use shared::constants::file::NOTIFIED_KEYS_LIMIT;
use shared::errors::{AppError, AppResult};
use shared::{ScrapedItem, ScraperOutput, State, UpdateHistory};

use tracing::{error, info};

/// デバッグ用関数
pub fn debug() {
  println!("Hello from kernel!");
}

/// initialize実行関数
pub fn run_initialize(config: &AppConfig) -> AppResult<()> {
  // init処理開始をdiscordに送信
  logger::log(log_info!("prune", "initialize処理実行開始"));

  // パスを持っておく
  let data_dir_path = config.data.dir_path.as_path();

  // ----------------------
  // dataディレクトリのリセット
  // ----------------------
  // dataが存在したらディレクトリごと削除
  if data_dir_path.exists() {
    match fs::remove_dir_all(data_dir_path) {
      Ok(_) => (),
      Err(e) => {
        logger::log(log_error!("initialize", "dataフォルダ削除失敗"));
        return Err(AppError::Process(format!("dataフォルダの削除失敗: {}", e)));
      }
    };
  }

  // ディレクトリを再作成（中間ディレクトリも含めて作成）
  match fs::create_dir_all(data_dir_path) {
    Ok(_) => (),
    Err(e) => {
      logger::log(log_error!("initialize", "dataフォルダ作成失敗"));
      return Err(AppError::Process(format!("dataフォルダの作成失敗: {}", e)));
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
      logger::log(log_error!("initialize", "スクレイピング失敗"));

      // 終了処理
      finish(config)?;

      // Err返し終了
      return Err(e);
    }
  };

  // ----------------------
  // 初期stateの作成
  // ----------------------
  let mut state = State {
    creat_state_at: output.fetched_at,
    last_update_at: output.fetched_at,
    last_seen_item: ScrapedItem::new(),
    notified_item_keys: Vec::new(),
  };

  // ----------------------
  // 更新差分判定(すべて新規)
  // ----------------------
  // Hash生成と取得したItemを認識
  let new_data: Vec<(String, ScrapedItem)> = match monitor::detect_new_item(&output, &state) {
    Ok(n) => n,
    Err(e) => {
      logger::log(log_error!("initialize", "差分判定(すべて新規)の失敗"));
      return Err(e);
    }
  };

  // タプルベクトルの戻り値を各ベクトルに分解
  let (fingerprints, scraped_items): (Vec<String>, Vec<ScrapedItem>) = new_data.into_iter().unzip();

  // ----------------------
  // state.jsonの作成
  // ----------------------
  // 初期stateに追加(一応中身があるかのif文)
  if let Some(item) = scraped_items.first() {
    state.notified_item_keys.extend(fingerprints);
    state.last_seen_item = item.clone();
  }
  // stateの書き込み
  if let Err(e) = infra::storage::save_state(data_dir_path, &state) {
    logger::log(log_error!("initialize", "stateの書き込み失敗"));
    return Err(e);
  }

  // ----------------------
  // detect_history作成
  // ----------------------
  if let Err(e) = infra::storage::append_detect_history(
    data_dir_path,
    &shared::DetectHistory {
      detected_at: output.fetched_at,
      updated: true,
    },
  ) {
    logger::log(log_error!("initialize", "detect_history作成失敗"));
    return Err(e);
  }

  // ----------------------
  // update_history作成
  // ----------------------
  // 一旦初期updatehistoryを作成
  let mut update_history = UpdateHistory::new();
  // 時間だけ代入
  update_history.detected_at = output.fetched_at;
  // 最新のやつだけupdate_historyに記入する
  if let Some(item) = scraped_items.into_iter().last() {
    update_history.ticker_symbol = item.ticker_symbol;
    update_history.ticker_name = item.ticker_name;
    update_history.published_at = item.published_at;
    update_history.title = item.title;
    update_history.url = item.url;
  }
  // update_history作成
  if let Err(e) = infra::storage::append_update_history(data_dir_path, &update_history) {
    logger::log(log_error!("initialize", "update_history作成失敗"));
    return Err(e);
  }

  // initしたことをdiscordに送信
  logger::log(log_info!("prune", "initialize処理実行完了"));

  // 終了処理
  finish(config)?;

  Ok(())
}

/// monitor実行関数
pub fn run_monitor(config: &AppConfig) -> AppResult<()> {
  // デバッグ用
  shared::debug();
  monitor::debug();
  discord::debug();

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
      &shared::UpdateHistory {
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

/// prune実行関数
pub fn run_prune(config: &AppConfig) -> AppResult<()> {
  // 調整することをdiscordに送信
  logger::log(log_info!("prune", "prune処理実行開始"));

  // detect_history.jsonlの調整
  if let Err(e) = infra::storage::prune_detect_history(config.data.dir_path.as_path()) {
    logger::log(log_error!("prune", "detect_historyの圧縮失敗"));
    return Err(e);
  }
  // update_history.jsonlの調整
  if let Err(e) = infra::storage::prune_update_history(config.data.dir_path.as_path()) {
    logger::log(log_error!("prune", "update_historyの圧縮失敗"));
    return Err(e);
  }

  // 調整したことをdiscordに送信
  logger::log(log_info!("prune", "prune処理実行完了"));

  // 終了処理
  finish(config)?;

  Ok(())
}

/// 終了時に実行する処理
fn finish(config: &AppConfig) -> AppResult<()> {
  // loggerをdiscord送信
  if let Err(e) = discord::send_logs(&config.discord.logs_webhook) {
    error!("ログの送信に失敗");
    return Err(e);
  }

  info!("終了処理を実行しました");

  Ok(())
}
