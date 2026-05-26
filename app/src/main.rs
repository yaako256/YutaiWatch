/*
app/src/main.rs
コマンド引数を受け取り、kernelに処理を委譲する
*/
// 内部ライブラリ(別クレート)
// kernel
use kernel;
// config用
use infra_config;
// エラー型用
use shared::errors::{AppError, AppResult};

// 外部ライブラリ
// ログ出力用
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() -> AppResult<()> {
  // ----------------------
  // 初期設定
  // ----------------------
  // ログ出力の設定
  // 環境変数 RUST_LOG からログレベルを読み込み、無ければデフォルトで「info」にする
  // ファイル出力とかをすぐ増やせて拡張性ましまし、最近流行の定義方法らしい
  let log_file = std::fs::File::create("app.log")
    .map_err(|e| AppError::Config(format!("ログファイル作成失敗: {}", e)))?;
  tracing_subscriber::registry()
    // ターミナルログ出力定義。
    .with(fmt::layer().with_writer(std::io::stdout).with_ansi(true))
    // ログファイル出力定義。
    .with(fmt::layer().with_writer(log_file).with_ansi(false))
    // 標準出力の設定。
    .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
    .init();

  // configのロード
  let config = infra_config::load_config()?;

  // ----------------------
  // 実行処理
  // ----------------------
  // コマンドの引数をパース
  // args[0] はプログラム名なのでスキップ
  let args: Vec<String> = std::env::args().skip(1).collect();

  // "--" が含まれる場合はその後ろを、ない場合はそのまま使う
  // - 開発時: cargo run -p app -- monitor → Cargoが"--"を消費 → ["monitor"]
  // - 本番時: ./aaa -- monitor            → プログラムに"--"が届く → ["--", "monitor"]
  let command_args: Vec<&str> = {
    match args.iter().position(|a| a == "--") {
      Some(i) => args[i + 1..].iter().map(|s| s.as_ref()).collect(),
      None => args.iter().map(|s| s.as_ref()).collect(),
    }
  };

  // 引数にあった関数を実行
  match command_args.as_slice() {
    //"initialize" => kernel::run_initialize(&config),
    ["monitor"] => kernel::run_monitor(&config),
    [] => Err(AppError::InvalidCommand(
      "コマンドを指定してください".into(),
    )),
    _ => Err(AppError::InvalidCommand("不明なコマンド".into())),
  }
}
