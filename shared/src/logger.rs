use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Logger(Arc<Mutex<String>>);

impl Logger {
  pub fn new() -> Self {
    Self(Arc::new(Mutex::new(String::new())))
  }

  // ログ追加
  pub fn log(&self, msg: impl AsRef<str>) {
    let mut buf = self.0.lock().unwrap();
    buf.push_str(msg.as_ref());
    buf.push('\n');
  }

  /// Discord送信用に2000文字以内で分割
  pub fn to_discord_chunks(&self) -> Vec<String> {
    let buf = self.0.lock().unwrap();
    let mut chunks = Vec::new();
    let mut current = String::new();

    for line in buf.lines() {
      if current.len() + line.len() + 1 > 2000 {
        chunks.push(format!("```\n{}\n```", current));
        current = String::new();
      }
      current.push_str(line);
      current.push('\n');
    }

    if !current.is_empty() {
      chunks.push(format!("```\n{}\n```", current));
    }

    chunks
  }
}
