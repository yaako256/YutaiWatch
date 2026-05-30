/*
kernel/src/initialize.rs
initialize処理の関数を定義
*/
// 標準ライブラリ
// ファイルIO用
use std::fs;

// Workspace内クレート
use infra_config::AppConfig;
use logger::{self, log_error, log_info};
use shared::{
  ScrapedItem, ScraperOutput, State, UpdateHistory,
  errors::{AppError, AppResult},
};

// 自クレート
use super::common::finish;

/// initialize実行関数
pub fn run_initialize(config: &AppConfig) -> AppResult<()> {
  // init処理開始をdiscordに送信
  logger::log(log_info!("initialize", "initialize処理実行開始"));

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
  // あらかじめ新規item数を取得
  let new_item_num = fingerprints.len();

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
      new_items: new_item_num,
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
  logger::log(log_info!("initialize", "initialize処理実行完了"));

  // 終了処理
  finish(config)?;

  Ok(())
}
