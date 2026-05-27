/*
infra/src/storage.rs
storageのIOを定義
*/
use std::{
  fs::{self, OpenOptions},
  io::{BufWriter, Write},
  path::Path,
};

use shared::constants;
use shared::errors::{AppError, AppResult};
use shared::{DetectHistory, State, UpdateHistory};

// エラー用
fn storage_error(path: &Path, action: &str, e: std::io::Error) -> AppError {
  AppError::Storage(std::io::Error::new(
    e.kind(),
    format!(
      "{action}に失敗しました: path={}, error={}",
      path.display(),
      e
    ),
  ))
}
// パースエラー用
fn parse_error(path: &Path, action: &str, e: impl std::fmt::Display) -> AppError {
  AppError::Parse(serde_json::Error::io(std::io::Error::new(
    std::io::ErrorKind::InvalidData,
    format!(
      "{action}に失敗しました: path={}, error={}",
      path.display(),
      e
    ),
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
