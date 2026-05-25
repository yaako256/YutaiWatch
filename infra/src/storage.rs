/*
infra/src/storage.rs
storageのIOを定義
*/
use shared::errors::AppResult;
use shared::{DetectHistory, State, UpdateHistory};

// state.json の読み込み（存在しない場合は None を返す）
//pub fn load_state(data_dir: &Path) -> AppResult<Option<State>>;

// state.json の書き込み
//pub fn save_state(data_dir: &Path, state: &State) -> AppResult<()>;

// detect_history.jsonl への追記
//pub fn append_detect_history(data_dir: &Path, entry: &DetectHistory) -> AppResult<()>;

// update_history.jsonl への追記
//pub fn append_update_history(data_dir: &Path, entry: &UpdateHistory) -> AppResult<()>;
