/*
infra_config\src\config\loader.rs
configをapp.tomlや.envからloadする関数の定義
`config/`はバイナリファイルと同じ階層(実行場所)に置く
*/
// 外部ライブラリ
// config用
use config::{Config, Environment, File};
// 内部ライブラリ
use crate::config::models::AppConfig;
use shared::errors::{AppError, AppResult};

pub fn load_config() -> AppResult<AppConfig> {
  // dotenv（開発用）
  dotenvy::from_path(".config/.env").ok();

  // 設定ファイルをロードする
  let settings = Config::builder()
    // 1. TOMLベース
    .add_source(File::with_name(".config/app.toml").required(false))
    // 2. ENV上書き（APP__PATHS__DATA_DIR_PATH形式）
    .add_source(
      Environment::with_prefix("APP")
        .separator("__")
        .try_parsing(true)
        .list_separator(","),
    )
    .build();

  // ロードできたか
  let settings = match settings {
    Ok(s) => s,
    Err(e) => return Err(AppError::Config(format!("{}", e))),
  };

  // AppConfigにデシリアライズ
  match settings.try_deserialize::<AppConfig>() {
    Ok(config) => return Ok(config),
    Err(e) => return Err(AppError::Config(format!("{}", e))),
  };
}
