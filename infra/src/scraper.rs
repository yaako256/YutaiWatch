/*
infra/src/scraper.rs
pythonを実行する部分
*/

use infra_config::ScraperConfig;

/// scraper pythonを実行する
pub fn run_scraper(config: &ScraperConfig) -> AppResult<ScraperOutput> {
  // → Command::new(python_command).arg(script_path).output()
  // → stdout を ScraperOutput に deserialize

  // scraper pythonを実行
  // その出力を取得
  let output = Command::new("python3")
    .arg("/app/scraper/main.py")
    .output()
    .expect("failed");
}
