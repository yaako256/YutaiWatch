pub mod app;
pub mod config;
pub mod domain;
pub mod error;
pub mod infra;

fn main() {
  // ログ表示を起動
  tracing_subscriber::fmt::init();

  println!("Hello, world!");
}
