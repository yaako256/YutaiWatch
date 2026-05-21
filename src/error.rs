use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  // HTTPリクエストに失敗した時のエラー
  #[error("HTTPリクエスト失敗")]
  HTTP,

  #[error("HTTPのパースに失敗")]
  Parse,

  #[error("データ保存に失敗")]
  Data,

  #[error("Discord通知に失敗")]
  Discord,
}
