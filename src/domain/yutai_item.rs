/*
src/domain/yutai_item.rs
優待情報の構造体を定義
*/

pub struct YutaiItem {
  pub stock_code: String,
  pub stock_name: String,
  pub published_at: String,
  pub scraped_at: String,
  pub benefit_title: String,
  pub detail_url: String,
}
