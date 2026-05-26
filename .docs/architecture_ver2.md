# architecture_ver2.mdについて
B4Pやってたら現状の構成が楽じゃなさそうなことがわかった。
どれが正解かはわからないが、B4Pと同じく、workspaceを分け、作ろうと思う。
そのため、設計書もそれ通りに変える。
あとスクレイピングをpythonでやることにしたので、それも記入する

# この設計書を作ってから変わった(かも)しれないところ
- configの比重を少なくし、定数などをsharedに置くことにした
- discordを本文のみではなく、embedを使うことにした。
- サイトにitem_keyがないため、内容ベースで自分で作ることにする。urlが擬似IDであるため、それを使って頑張る。

---

# YutaiWatch 設計書ver2

## 1. 概要

### 1.1 本プロジェクトについて

YutaiWatch は、株主優待速報サイトを定期監視し、新しい更新を検出した際に Discord へ通知するシステムである。
監視処理は単発実行型であり、定期実行は cron / supercronic など外部スケジューラに委ねる。

本システムでは、対象サイトの bot 対策や動的描画への耐性を重視し、**スクレイピング処理は Python** で実装する。
Rust は、Python の出力を受け取って差分判定・状態更新・Discord 通知・実行制御を担う。

---

## 2. 設計方針

### 2.1 責務分離

本システムは以下の 4 層で考える。

* **scraper（Python）**
  対象サイトから情報を取得し、正規化済みデータを標準出力へ返す。
* **monitor（Rust）**
  取得結果と保存済み状態を比較し、新規更新を判定する。
* **infra（Rust）**
  外部 I/O、ファイル保存、Python プロセス起動、ログ出力などを扱う。
* **discord（Rust）**
  Discord Webhook 送信を扱う。

### 2.2 設計の基本姿勢

* panic を避け、Result ベースで伝播する
* 通知漏れを最重要視する
* 外部要因による失敗を前提にする
* Python と Rust の境界を明確に保つ
* 後から対象サイトが増えても壊れにくい構造にする

### 2.3 旧設計からの主な見直し

* `HEAD / ETag` 前提は採用しない
  対象サイトは scraping 前提であり、ページ取得の安定性が最優先となるため、**取得結果そのものを比較する設計**に切り替える。
* HTML 解析を Rust に閉じ込めない
  解析は Python に寄せ、Rust は **正規化済みデータの処理**に専念する。
* 単一バイナリ前提をやめる
  Workspace によって、役割ごとに crate を分割する。

---

## 3. システム構成

### 3.1 実行方式

```text
cron / supercronic
   ↓
Docker 本番コンテナ起動
   ↓
Rust バイナリ起動
   ↓
Python スクレイパー実行
   ↓
JSON 受け取り
   ↓
差分判定
   ↓
Discord 通知
   ↓
状態保存
   ↓
終了
```

### 3.2 処理モデル

* 常駐アプリではなく、**1回の起動で 1 回分の監視を行う Batch 型**
* スケジュール管理は外部に委譲する
* 同一処理を繰り返し安全に実行できるようにする

---

## 4. Workspace 設計

### 4.1 全体構造

親クレート `YutaiWatch` は workspace ルートのみとし、実装は子 crate に分割する。

```text
YutaiWatch/
├── app/       # 全体実行用バイナリ
├── shared/    # 共通型・共通ユーティリティ
├── kernel/    # 実行制御・ユースケース起点
├── config/    # 設定ファイル読込
├── discord/   # Discord Webhook 送信
├── infra/     # 外部I/O、ファイル、Python起動など
└── monitor/   # 更新検知・差分判定・通知対象整理
```

### 4.2 各 crate の責務

| crate     | 責務                                           |
| --------- | -------------------------------------------- |
| `shared`  | 全 crate で共有する型、エラー、ID、日時正規化                  |
| `kernel`  | 監視フローの起動、処理順制御、失敗時の扱い                        |
| `config`  | `.config/app.toml` と `.config/.env` の読込、検証           |
| `discord` | Webhook 送信、通知メッセージ整形                         |
| `infra`   | Python subprocess 起動、ファイル保存、state 読込書込、ログ初期化 |
| `monitor` | スクレイプ結果と state の比較、更新抽出、通知対象の決定              |

### 4.3 推奨 workspace root

```toml
[workspace]
members = [
  "app",
  "shared",
  "kernel",
  "config",
  "discord",
  "infra",
  "monitor",
]
resolver = "2"
```

---

## 5. ディレクトリ構成

```text
YutaiWatch/
├── .dockerignore
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── compose.yaml
├── Makefile
├── requirements.txt
├── rustfmt.toml
├── scraper/
├── .config/
│   ├── app.toml
│   └── .env
├── docker/
│   ├── dev/
│   │   └── Dockerfile
│   └── prod/
│       ├── crontab
│       ├── Dockerfile
│       └── entrypoint.sh
├── app/
│   ├── main.rs
│   └── Cargo.toml
├── shared/
│   ├── lib.rs
│   └── Cargo.toml
├── kernel/
│   ├── lib.rs
│   └── Cargo.toml
├── discord/
│   ├── lib.rs
│   └── Cargo.toml
├── infra/
│   ├── lib.rs
│   └── Cargo.toml
└── monitor/
    ├── lib.rs
    └── Cargo.toml
```

---

## 6. Python スクレイパー設計

### 6.1 役割

Python は以下のみを担当する。

* 対象サイトへアクセスする
* bot 対策や動的描画に対応する
* 必要な HTML / DOM を解析する
* Rust が扱いやすい形へ正規化する
* 結果を stdout に JSON で出力する

### 6.2 Rust からの起動方法

Rust 側は subprocess として Python を起動する。

```rust
let output = Command::new("python3")
    .arg("/app/scraper/main.py")
    .output()
    .expect("failed");
```

### 6.3 stdout 契約

Python は **標準出力に JSON を 1 回だけ出す**。
標準エラーはログ用途とし、Rust は原則として stdout を正として扱う。

#### 出力例

```json
{
  "schema_version": 1,
  "fetched_at": "2026-05-24T08:00:00+09:00",
  "source": "yutai_sokuho",
  "items": [
    {
      "item_key": "20260524_001",
      "title": "○○の株主優待変更",
      "url": "https://example.com/...",
      "published_at": "2026-05-24T07:30:00+09:00",
      "company_code": "1234",
      "company_name": "○○株式会社"
    }
  ]
}
```

### 6.4 失敗時の扱い

* Python 終了コードが 0 以外なら失敗
* stdout が空、または JSON 解析不能なら失敗
* 失敗時は state を更新しない
* stderr は debug 用に保持する

### 6.5 契約の考え方

Python は「取得と整形」に集中し、**差分判定は Rust に持たせる**。
これにより、解析ロジック変更時の影響範囲を抑える。

---

## 7. 監視フロー

## 7.1 初回実行

初回実行は `state` が存在しない場合に行う。

```text
state 不在
↓
Python で最新取得
↓
state 作成
↓
初期スナップショット保存
↓
「監視開始」通知またはログ記録
↓
終了
```

### 7.2 通常実行

```text
起動
↓
設定読込
↓
state 読込
↓
Python 実行
↓
取得結果の検証
↓
最新スナップショット作成
↓
state と比較
↓
更新なし → ログ記録して終了
↓
更新あり → Discord 通知
↓
通知成功 → state 更新
↓
終了
```

### 7.3 通知漏れ防止

state 更新は **Discord 送信成功後のみ** 行う。
これにより、以下の不整合を防ぐ。

* state は進んだが通知していない
* 通知は失敗したが更新済み扱いになる

---

## 8. 状態管理

### 8.1 state の考え方

HEAD / ETag を前提にせず、**取得結果そのものを基準にする**。

### 8.2 state の保存内容

```json
{
  "schema_version": 1,
  "last_success_at": "2026-05-24T08:00:00+09:00",
  "last_snapshot_hash": "sha256:...",
  "last_seen_item_key": "20260524_001",
  "notified_item_keys": [
    "20260524_001",
    "20260523_004"
  ]
}
```

### 8.3 使い分け

* `last_snapshot_hash`
  前回取得との差分確認やデバッグに使う
* `last_seen_item_key`
  直近の基準点として使う
* `notified_item_keys`
  重複通知防止に使う

### 8.4 item_key の決め方

サイト側に安定した ID があるならそれを使う。
なければ以下のように生成する。

```text
item_key = sha256(normalized_title + normalized_url + published_at + company_code)
```

---

## 9. 永続化設計

### 9.1 ファイル方針

* state は小さな JSON として保存
* 実行履歴は JSONL に残す
* 人間向けログは別途ファイルに残す

### 9.2 保存先候補

```text
data/state.json
data/detect_history.jsonl
data/update_history.jsonl
logs/app.log
```

### 9.3 detect_history.jsonl

実行履歴を残す。

```json
{"detected_at":"2026-05-24T08:00:00+09:00","updated":false}
{"detected_at":"2026-05-24T08:10:00+09:00","updated":true}
```

### 9.4 update_history.jsonl

通知対象になった更新を残す。

```json
{
  "detected_at":"2026-05-24T08:10:00+09:00",
  "title":"○○の株主優待変更",
  "url":"https://example.com/...",
  "published_at":"2026-05-24T07:30:00+09:00",
  "company_code":"1234"
}
```

---

## 10. Discord 通知設計

### 10.1 通知内容

```text
1234 ○○株式会社
公開時刻: 2026-05-24 07:30
○○の株主優待変更
<https://example.com/...>
```

### 10.2 送信単位

* 原則は 1 回の監視で検出した更新をまとめて送る
* 長文になる場合は複数メッセージに分割する
* 送信失敗時は state を更新しない

### 10.3 失敗時の方針

* リトライを行う
* 最終失敗時はエラーとして終了する
* 次回実行で再検知できるようにする

---

## 11. エラー設計

### 11.1 エラー分類

```rust
enum AppError {
    Config,
    Scrape,
    Parse,
    Diff,
    Discord,
    Storage,
    Process,
}
```

### 11.2 方針

* panic を原則禁止
* 失敗地点と理由をログに残す
* Python 実行失敗と JSON 解析失敗は区別する
* 更新検知に失敗しても、次回の再実行に支障を残さない

---

## 12. ログ設計

### 12.1 ログレベル

* INFO
* WARN
* ERROR

### 12.2 必ず記録するもの

* 起動
* 設定読込結果
* Python 実行結果
* 更新有無
* Discord 送信結果
* state 更新結果
* 失敗時の原因

---

## 13. Docker 設計

### 13.1 方針

Docker は以下の 2 系統で運用する。

* **開発用コンテナ**
  Rust の開発、テスト、`cargo watch`
* **本番用コンテナ**
  Rust バイナリを常駐させ、supercronic で定期実行

### 13.2 開発用

* Rust
* Python
* Playwright
* cargo-watch

### 13.3 本番用

* Rust バイナリ
* Python 実行環境
* scraper ディレクトリ
* cron 実行環境

### 13.4 運用イメージ

```text
compose.yaml
  ├─ yutai_watch_dev
  └─ yutai_watch
```

### 13.5 cron 設計

```cron
*/10 8-17 * * 1-5 /app/yutai_watch
*/30 0-7,18-23 * * * /app/yutai_watch
*/30 * * * 0,6 /app/yutai_watch
```

---

## 14. CLI 設計

### 14.1 app crate

`app` は全体実行用のバイナリクレートとする。

### 14.2 想定コマンド

* `initialize`
* `monitor`

例:

```bash
cargo run -p app -- initialize
cargo run -p app -- monitor
```

ただし実運用では Docker 内でバイナリを直接実行する。

---

## 15. 将来拡張

* 複数サイト監視
* Python スクレイパーの複数化
* SQLite への移行
* Slack 通知対応
* 通知テンプレートの差し替え
* GitHub Actions での補助実行
* 監視対象の設定ファイル化

---

## 16. この設計の要点

この改訂版の核は、次の 3 点です。

1. **スクレイピングは Python に任せる**
   対象サイトの対策が強くても対応しやすい。
2. **Rust Workspace で役割を分離する**
   監視、設定、通知、I/O を独立させて保守性を上げる。
3. **Docker と cron を前提に運用する**
   本番実行を単純化し、再起動・再実行にも強くする。
