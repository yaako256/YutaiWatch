use std::fs;

use infra_config::AppConfig;
use logger;
use shared::constants::file::NOTIFIED_KEYS_LIMIT;
use shared::errors::{AppError, AppResult};
use shared::{ScrapedItem, ScraperOutput, State, UpdateHistory};

// 外部ライブラリ
// ログ出力
use tracing::error;

/// デバッグ用関数
pub fn debug() {
  println!("Hello from kernel!");
}

/// initialize実行関数
pub fn run_initialize(config: &AppConfig) -> AppResult<()> {
  // パスを持っておく
  let data_dir_path = config.data.dir_path.as_path();

  // ----------------------
  // dataディレクトリのリセット
  // ----------------------
  // dataが存在したらディレクトリごと削除
  if data_dir_path.exists() {
    match fs::remove_dir_all(data_dir_path) {
      Ok(_) => (),
      Err(e) => return Err(AppError::Process(format!("dataフォルダの削除失敗: {}", e))),
    };
  }
  logger::log("めっせーじ");
  // ディレクトリを再作成（中間ディレクトリも含めて作成）
  match fs::create_dir_all(data_dir_path) {
    Ok(_) => (),
    Err(e) => return Err(AppError::Process(format!("dataフォルダの作成失敗: {}", e))),
  };

  // ----------------------
  // 監視対象サイトをスクレイピング
  // pythonを実行
  // ----------------------
  // スクレイピング部分実行
  let output: ScraperOutput = match infra::scraper::run_scraper(&config.scraper) {
    Ok(o) => o,
    Err(e) => {
      // 終了処理(未実装)
      // finish()

      // Err返し終了
      return Err(AppError::Process(format!("{}", e)));
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
  let new_data: Vec<(String, ScrapedItem)> = monitor::detect_new_item(&output, &state)?;

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
  infra::storage::save_state(data_dir_path, &state)?;

  // ----------------------
  // detect_history作成
  // ----------------------
  infra::storage::append_detect_history(
    data_dir_path,
    &shared::DetectHistory {
      detected_at: output.fetched_at,
      updated: true,
    },
  )?;

  // ----------------------
  // update_history作成
  // ----------------------
  // 一旦初期updatehistoryを作成
  let mut update_history = UpdateHistory::new();
  // 時間だけ代入
  update_history.detected_at = output.fetched_at;
  // 最新のやつだけupdate_historyに記入
  if let Some(item) = scraped_items.into_iter().last() {
    update_history.ticker_symbol = item.ticker_symbol;
    update_history.ticker_name = item.ticker_name;
    update_history.published_at = item.published_at;
    update_history.title = item.title;
    update_history.url = item.url;
  }
  // update_historyに記入
  infra::storage::append_update_history(data_dir_path, &update_history)?;

  // initしたことをdiscordに送信(未実装)
  // discord::send_notify(&config.discord.notify_webhook, &scraped_items)?;

  // 終了処理(未実装)
  // finish()

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
      // デバッグログ
      error!("Stateの取得に失敗。initフローの実行をします:{}", e);

      // 初回フローの実行
      run_initialize(config)?;

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
      // デバッグログ
      //error!("Stateに値なし。initフローの実行をしましす:{}", e);

      // 初回フローの実行
      run_initialize(config)?;

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
      // 終了処理(未実装)
      // finish()

      // Err返し終了
      return Err(AppError::Process(format!("{}", e)));
    }
  };

  // ----------------------
  // 更新差分判定
  // 更新なし → 終了処理
  // 更新あり → 処理実行
  // ----------------------
  let new_data: Vec<(String, ScrapedItem)> = monitor::detect_new_item(&output, &state)?;

  // 空だったらdetect_history更新だけして実行終了
  if new_data.is_empty() {
    // detect_historyの更新
    infra::storage::append_detect_history(
      config.data.dir_path.as_path(),
      &shared::DetectHistory {
        detected_at: output.fetched_at,
        updated: false,
      },
    )?;

    // 終了処理(未実装)
    // finish()

    // Okを返し終了
    return Ok(());
  }

  // タプルベクトルの戻り値を各ベクトルに分解
  let (fingerprints, scraped_items): (Vec<String>, Vec<ScrapedItem>) = new_data.into_iter().unzip();

  // ----------------------
  // 更新内容を通知
  // ----------------------
  // discordに送信
  discord::send_notify(&config.discord.notify_webhook, &scraped_items)?;

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
  infra::storage::save_state(config.data.dir_path.as_path(), &state)?;

  // ----------------------
  // detect_history更新
  // ----------------------
  infra::storage::append_detect_history(
    config.data.dir_path.as_path(),
    &shared::DetectHistory {
      detected_at: output.fetched_at,
      updated: true,
    },
  )?;

  // ----------------------
  // update_history更新
  // ----------------------
  for item in scraped_items {
    infra::storage::append_update_history(
      config.data.dir_path.as_path(),
      &shared::UpdateHistory {
        detected_at: output.fetched_at,
        ticker_symbol: item.ticker_symbol,
        ticker_name: item.ticker_name,
        published_at: item.published_at,
        title: item.title,
        url: item.url,
      },
    )?;
  }

  // 終了処理(未実装)
  // finish()

  Ok(())
}

/// prune実行関数
pub fn run_prune(config: &AppConfig) -> AppResult<()> {
  // detect_history.jsonlの調整
  infra::storage::prune_detect_history(config.data.dir_path.as_path())?;
  // update_history.jsonlの調整
  infra::storage::prune_update_history(config.data.dir_path.as_path())?;

  // 調整したことをdiscordに送信(未実装)

  Ok(())
}
