pub mod app;
pub mod config;
pub mod domain;
pub mod error;
pub mod infra;

use reqwest::blocking::Client;

fn main() {
  // ログ表示を起動
  tracing_subscriber::fmt::init();

  println!("Hello, world!");
  /*
   let client = Client::new();

   let response = client
     .head("https://www.invest-jp.net/yuutai/news")
     .send()?;

   println!("Status: {}", response.status());

   // ETag
   if let Some(etag) = response.headers().get(ETAG) {
     println!("ETag: {}", etag.to_str()?);
   }

   // Last-Modified
   if let Some(last_modified) = response.headers().get(LAST_MODIFIED) {
     println!("Last-Modified: {}", last_modified.to_str()?);
   }
  */
  //Ok(())
}
