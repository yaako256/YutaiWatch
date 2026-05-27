/*
shared/src/constants/file.rs
ファイルの定数設定
*/

// データファイル名
pub const STATE_FILE_NAME: &str = "state.json";
pub const DETECT_HISTORY_FILE_NAME: &str = "detect_history.jsonl";
pub const UPDATE_HISTORY_FILE_NAME: &str = "update_history.jsonl";

// state.json関連
// 保存時に上限を超えたら古い順に切り捨てる
pub const NOTIFIED_KEYS_LIMIT: usize = 200;

// detect_history.jsonl関連
// 保存項目の上限
pub const DETECT_HISTORY_LIMIT: usize = 10000;

// update_history.jsonl関連
// 保存項目の上限
pub const UPDATE_HISTORY_LIMIT: usize = 1000;
