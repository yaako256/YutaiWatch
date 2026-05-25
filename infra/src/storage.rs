use std::fs;
use std::path::PathBuf;

/*
infra/src/storage.rs
storageのIOを定義
*/
use shared::constants;
use shared::errors::{AppError, AppResult};
use shared::{DetectHistory, State, UpdateHistory};

// state.json の読み込み（存在しない場合は None を返す）
pub fn load_state(data_dir: &PathBuf) -> AppResult<Option<State>> {
  let state_path = data_dir.join(constants::file::STATE_FILE_NAME);

  // ファイルが存在しない
  if !state_path.exists() {
    return Ok(None);
  }

  // ファイル読み込み
  let json_text = fs::read_to_string(&state_path).map_err(|e| {
    AppError::Storage(std::io::Error::new(
      e.kind(),
      format!(
        "state.json の読み込みに失敗しました: path={}, error={}",
        state_path.display(),
        e
      ),
    ))
  })?;

  // JSON -> State にデシリアライズ
  let state: State = serde_json::from_str(&json_text).map_err(|e| {
    AppError::Parse(serde_json::Error::io(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      format!(
        "state.json のJSONパースに失敗しました: path={}, error={}",
        state_path.display(),
        e
      ),
    )))
  })?;

  Ok(Some(state))
}

// state.json の書き込み
//pub fn save_state(data_dir: &Path, state: &State) -> AppResult<()>;

// detect_history.jsonl への追記
//pub fn append_detect_history(data_dir: &Path, entry: &DetectHistory) -> AppResult<()>;

// update_history.jsonl への追記
//pub fn append_update_history(data_dir: &Path, entry: &UpdateHistory) -> AppResult<()>;
