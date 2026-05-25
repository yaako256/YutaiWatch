/*
infra_config\src\lib.rs

infra_configは、
- 設定のロード
- 設定の構造体定義
を司るクレート
*/
mod config;

// 再エクスポート
pub use config::{AppConfig, load_config};

// デバッグ用関数
pub fn debug() {
  println!("hello infra_config");
}
