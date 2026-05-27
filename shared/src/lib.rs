pub mod constants;
pub mod errors;
pub mod logger;
mod models;

// モデルは階層を浅くしようかな
pub use models::detect_history::DetectHistory;
pub use models::scraper_output::*;
pub use models::state::State;
pub use models::update_history::UpdateHistory;

// デバッグ用関数
pub fn debug() {
  println!("Hello from shared!");
}
