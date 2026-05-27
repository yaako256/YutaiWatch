/*
logger/src/lib.rs
グローバルで持つloggerの定義
*/
use std::sync::{Mutex, OnceLock};

static LOGGER: OnceLock<Mutex<String>> = OnceLock::new();

/// loggerの起動
pub fn init() {
  LOGGER.set(Mutex::new(String::new())).unwrap();
}

/// log追加
pub fn log(msg: impl AsRef<str>) {
  let mut buf = LOGGER.get().unwrap().lock().unwrap();
  buf.push_str(msg.as_ref());
  buf.push('\n');
}

/// printする
pub fn print() {
  let buf = LOGGER.get().unwrap().lock().unwrap();
  print!("{}", *buf);
}

/// loggerを指定文字数ごとに区切る
pub fn to_chunks(limit: usize) -> Vec<String> {
  let buf = LOGGER.get().unwrap().lock().unwrap();
  let mut chunks = Vec::new();
  let mut current = String::new();

  for line in buf.lines() {
    if current.len() + line.len() + 1 > limit {
      chunks.push(current.clone());
      current = String::new();
    }
    current.push_str(line);
    current.push('\n');
  }

  if !current.is_empty() {
    chunks.push(current);
  }

  chunks
}
