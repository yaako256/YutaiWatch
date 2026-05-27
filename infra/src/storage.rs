/*
infra/src/storage.rs
storageのIOを定義
*/
use std::{
  fs::{self, File, OpenOptions},
  io::{BufRead, BufReader, BufWriter, Write},
  path::Path,
};

use logger::{log_error, log_info, log_warn};
use shared::constants;
use shared::errors::{AppError, AppResult};
use shared::{DetectHistory, State, UpdateHistory};

// エラー用
fn storage_error(path: &Path, action: &str, e: std::io::Error) -> AppError {
  let msg = format!("{action}に失敗しました: path={}, error={e}", path.display());
  logger::log(log_error!("storage", msg));
  AppError::Storage(std::io::Error::new(e.kind(), msg))
}
// パースエラー用
fn parse_error(path: &Path, action: &str, e: impl std::fmt::Display) -> AppError {
  let msg = format!("{action}に失敗しました: path={}, error={e}", path.display());
  logger::log(log_error!("storage", msg));
  AppError::Parse(serde_json::Error::io(std::io::Error::new(
    std::io::ErrorKind::InvalidData,
    msg,
  )))
}

// state.json の読み込み（存在しない場合は None を返す）
pub fn load_state(data_dir: &Path) -> AppResult<Option<State>> {
  let state_path = data_dir.join(constants::file::STATE_FILE_NAME);

  // ファイルが存在しない場合
  if !state_path.exists() {
    return Ok(None);
  }

  // ファイル読み込み
  let json_text = fs::read_to_string(&state_path)
    .map_err(|e| storage_error(&state_path, "state.json の読み込み", e))?;

  // JSON -> State にデシリアライズ
  let state = serde_json::from_str(&json_text)
    .map_err(|e| parse_error(&state_path, "state.json のJSONパース", e))?;

  Ok(Some(state))
}

// state.json の書き込み
pub fn save_state(data_dir: &Path, state: &State) -> AppResult<()> {
  let state_path = data_dir.join(constants::file::STATE_FILE_NAME);

  // pretty json 化
  let json_text = serde_json::to_string_pretty(state)
    .map_err(|e| parse_error(&state_path, "State のJSONシリアライズ", e))?;

  // 書き込み
  fs::write(&state_path, json_text)
    .map_err(|e| storage_error(&state_path, "state.json の書き込み", e))?;

  Ok(())
}

// -------------------------
// jsonlの追記関連
// -------------------------
// jsonl追記の共通関数
fn append_jsonl<T: serde::Serialize>(path: &Path, entry: &T, label: &str) -> AppResult<()> {
  // ファイルを開く
  let file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(path)
    .map_err(|e| storage_error(path, &format!("{label} のオープン"), e))?;

  // 追記モードにする
  let mut writer = BufWriter::new(file);

  // JSONシリアライズ
  serde_json::to_writer(&mut writer, entry)
    .map_err(|e| parse_error(path, &format!("{label} のJSONシリアライズ"), e))?;

  // 追記する
  writer
    .write_all(b"\n")
    .map_err(|e| storage_error(path, &format!("{label} の追記"), e))?;

  Ok(())
}

// detect_history.jsonl への追記
pub fn append_detect_history(data_dir: &Path, entry: &DetectHistory) -> AppResult<()> {
  let path = data_dir.join(constants::file::DETECT_HISTORY_FILE_NAME);
  append_jsonl(&path, entry, constants::file::DETECT_HISTORY_FILE_NAME)
}

// update_history.jsonl への追記
pub fn append_update_history(data_dir: &Path, entry: &UpdateHistory) -> AppResult<()> {
  let path = data_dir.join(constants::file::UPDATE_HISTORY_FILE_NAME);
  append_jsonl(&path, entry, constants::file::UPDATE_HISTORY_FILE_NAME)
}

// -------------------------
// jsonlのデータ整理関連
// -------------------------
// jsonl のエントリ数を上限に揃える共通関数
fn prune_jsonl(path: &Path, limit: usize, label: &str) -> AppResult<()> {
  // ファイルが存在しない場合はスキップ
  if !path.exists() {
    // fileが存在しなかったことをlogに入れる
    let msg = format!("fileが存在しません label:{}", label);
    logger::log(log_warn!("prune_jsonl", msg));

    return Ok(());
  }

  // 全行を読み込む
  let file =
    File::open(path).map_err(|e| storage_error(path, &format!("{label} のオープン"), e))?;
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader
    .lines()
    .collect::<Result<_, _>>()
    .map_err(|e| storage_error(path, &format!("{label} の読み込み"), e))?;

  let total = lines.len();

  // 上限以下なら何もしない
  if total <= limit {
    // 何もしなかったことをlogに入れる
    let msg = format!("prune処理意味なし label:{}", label);
    logger::log(log_info!("prune_jsonl", msg));

    return Ok(());
  }

  // 古い方（先頭）を捨て、新しい方（末尾）を残す
  let pruned_count = total - limit;
  let kept_lines = &lines[pruned_count..];

  // 一時ファイルに書き出してからリネーム（書き込み中のクラッシュ対策）
  let tmp_path = path.with_extension("tmp");
  {
    let tmp_file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(&tmp_path)
      .map_err(|e| storage_error(&tmp_path, &format!("{label} の一時ファイルオープン"), e))?;
    let mut writer = BufWriter::new(tmp_file);
    for line in kept_lines {
      writer
        .write_all(line.as_bytes())
        .map_err(|e| storage_error(&tmp_path, &format!("{label} の書き込み"), e))?;
      writer
        .write_all(b"\n")
        .map_err(|e| storage_error(&tmp_path, &format!("{label} の書き込み"), e))?;
    }
  }

  fs::rename(&tmp_path, path)
    .map_err(|e| storage_error(path, &format!("{label} のリネーム"), e))?;

  // どのくらい改変したかをlogに残す
  let msg = format!(
    "prune処理実行 pruned_count:{} label:{}",
    pruned_count, label
  );
  logger::log(log_info!("prune_jsonl", msg));

  Ok(())
}

// detect_history.jsonl の刈り込み
pub fn prune_detect_history(data_dir: &Path) -> AppResult<()> {
  let path = data_dir.join(constants::file::DETECT_HISTORY_FILE_NAME);
  prune_jsonl(
    &path,
    constants::file::DETECT_HISTORY_LIMIT,
    constants::file::DETECT_HISTORY_FILE_NAME,
  )
}

// update_history.jsonl の刈り込み
pub fn prune_update_history(data_dir: &Path) -> AppResult<()> {
  let path = data_dir.join(constants::file::UPDATE_HISTORY_FILE_NAME);
  prune_jsonl(
    &path,
    constants::file::UPDATE_HISTORY_LIMIT,
    constants::file::UPDATE_HISTORY_FILE_NAME,
  )
}
