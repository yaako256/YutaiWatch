現在Rustプロジェクトの設計書を考えています。次のようなことをするRustプロジェクトを作成しようとしています。完璧な設計書を作るための助言を提示、その設計書を作成してください。

# やりたいこと
- 株主優待速報のサイトを定期監視、変更点がありそうだったらその旨をdiscordに通知する。
- 定期監視の方法は、Rustバイナリをcronで実行するため、Rust側に定期実行機能はつけない。
- 初回実行時は変更も何もないため、現状を取得するというプログラムを実行する。
- 検出記録ファイルと更新記録ファイルとログファイルを用意、textやjsonなどで保持をする。
- 初回以降は以下のフローチャートに沿って実行をする。
```text
# 実行フローチャート
cronでRustバイナリ起動
↓
HTTPのヘッダリクエストで、前回起動時以降に更新があったかを確認する。
変更無→検出記録ファイルに検出時間だけを記録。ログファイルに検出時間と更新がなかったことを記録。実行を終了する。
↓
HTTPリクエストを送信。対象部分のうち、新しい項目を見つける。
その項目のタイトルや時間、リンクやサイトの更新時間を取得
↓
検出記録ファイルに検出時間を記録。更新記録ファイルに更新内容を記録。
↓
discord通知用に成形
↓
discordに送信
↓
ちゃんと送信までできたことをログファイルに残す。
```
- 他プロジェクトの練習も兼ねているため、書き途中の設計書に示した設計思想を採用する。

以下に現在作成途中の設計書を示します。


# YutaiWatch 設計書
# 1. 概要
## 1.1 本プロジェクトについて
YutaiWatch は、株主優待速報サイトを定期監視し、新しい更新を検出した際に Discord へ通知を送信する Rust 製 CLI アプリケーションである。  
本システムは cron 等の外部スケジューラから定期起動されることを前提としており、アプリケーション内部には定期実行機能を持たない。

---

# 2. システム目的
本システムの目的は以下。
* 株主優待速報サイトの変更検知
* 更新情報の抽出
* 更新履歴の保存
* Discord通知
* 実行ログ保存
* 通知漏れ防止
* 安全な再実行

---
# 3. システム構成
## 3.1 実行方式
```text
cron
 ↓
Rust Binary 起動
 ↓
監視処理実行
 ↓
終了
```
単発実行型（Batch型）CLIアプリケーションとして設計する。

---
# 4. 設計思想
## 4.1 Lightweight Layered Architecture
本システムでは責務分離と保守性向上を目的として、軽量なレイヤ分離を採用する。

### app
ユースケースを実装する。  
例：
* initialize
* monitor

---
### domain
ビジネスルールを保持する。  
例：
* 差分検出
* 更新判定
* データ整形

---
### infra
外部システムアクセスを担当する。  
例：
* HTTP通信
* HTML解析
* Discord通知
* File IO
* Logging

---
## 4.2 Simple First ← ここは設計的にはいらんやろ。LLMに他プロジェクトの設計練習が云々って言ったせいで生まれた
本システムは単一責務・単一バイナリのCLIである。  
そのため以下は採用しない。
* Actor Model
* Event Bus
* CQRS
* Microservice
* 過剰なtrait抽象
* 過剰DI

可読性・保守性・運用容易性を優先する。

---

## 4.3 Failure-Oriented Design
外部通信を行うため、失敗を前提として設計する。
### 想定障害
* HTTP Timeout
* 一時的な通信失敗
* Discord送信失敗
* HTML構造変更
* JSON破損
* File IO失敗

### 基本方針
* panicを避ける
* Resultでエラー伝播
* recover可能なものはretry
* state破壊を防止
* 通知漏れを最重要視

---
# 5. ディレクトリ構成
```text
yutaiwatch/
├── Cargo.toml
├── .env
├── data/
│   ├── state.json
│   ├── detect_history.jsonl
│   └── update_history.jsonl
├── logs/
│   └── app.log
└── src/
    ├── main.rs
    ├── app/
    │   ├── initialize.rs
    │   └── monitor.rs
    ├── domain/
    │   ├── yutai_item.rs
    │   ├── state.rs
    │   └── diff.rs
    ├── infra/
    │   ├── http.rs
    │   ├── parser.rs
    │   ├── discord.rs
    │   ├── storage.rs
    │   └── logger.rs
    ├── config.rs
    └── error.rs
```

---
# 6. 初回起動設計
## 6.1 初回起動判定
以下条件のいずれかを満たす場合、初回起動とみなす。
* `state.json`が存在しない
* `state.json`が空
* `state.json`のdeserialize失敗

専用の「初回フラグ」は持たない。

---
## 6.2 初回実行フロー
```text
initialize 実行
↓
サイトから現状取得
↓
state.json 作成
↓
detect_history に初期化記録
↓
discordにモニタ開始を通知
↓
終了
```

---
# 7. 通常監視フロー
```text
cronで起動
↓
state.json 読み込み
↓
HEADリクエスト送信
↓
Last-Modified / ETag 比較

変更なし
 ├ detect_history.jsonl に記録
 ├ log出力
 └ 終了

変更あり
↓
GETリクエスト
↓
HTML解析
↓
新規更新抽出
↓
情報を通知用にフォーマット
↓
Discord通知
↓
通知成功
 ├ update_history.jsonl 保存
 ├ state.json 更新
 ├ detect_history.jsonl 記録
 └ log出力

通知失敗
 ├ state更新しない
 ├ error log
 └ 終了
```

---

# 8. 状態管理設計
## 8.1 state.json
監視状態を保持する。
内容は実際にヘッダリクエストを確認してから決める
```json
{
  "last_modified": "Wed, 21 May 2026 10:00:00 GMT",
  "etag": "abc123",
  "last_item_id": "20260521_001"
}
```

---

## 8.2 state更新タイミング
通知漏れ防止のため、`state.json` の更新は Discord送信成功後にのみ実施する。
これにより、
```text
stateだけ更新され通知されない
```
状態を防止する。

---
# 9. 永続化設計
## 9.1 detect_history.jsonl
監視実行履歴。
JSON Lines形式を採用する。
### 例
```json id="7l5lto"
{"detected_at":"2026-05-21T10:00:00+09:00","updated":false}
{"detected_at":"2026-05-21T11:00:00+09:00","updated":true}
```

---
## 9.2 update_history.jsonl
更新検知履歴。
### 例
```json id="mxy0jc"
{
  "detected_at":"2026-05-21T11:00:00+09:00",
  "title":"○○の株主優待変更",
  "url":"https://example.com/...",
  "published_at":"2026-05-21T10:30:00+09:00"
}
```

---
## 9.3 app.log
人間向けログ。
### 例
```text
[INFO] monitor started
[INFO] no updates
[INFO] update detected
[INFO] discord notification sent
[ERROR] failed to parse html
```

---

# 10. ドメイン設計
## 10.1 YutaiItem
株主優待更新情報を表す構造体。
### 属性
* 証券コード
* 銘柄名
* published_at
* title
* url
* item_id

---

## 10.2 State
監視状態。
### 属性
* last_modified
* etag
* last_item_id

---
## 10.3 Diff Logic
現在取得した更新情報と過去stateを比較し、新規更新のみ抽出する。

---
# 11. HTTP設計
## 11.1 HEADリクエスト
更新有無確認用途。
### 使用ヘッダ
* Last-Modified
* ETag

---

## 11.2 GETリクエスト
更新時のみ実施。
HTML本文取得用途。

---
## 11.3 retry方針
一時的通信失敗時は retry を行う。
### 対象
* timeout
* 5xx
* connection reset

### 非対象
* 4xx
* parse失敗

---

# 12. HTML解析設計
## 12.1 Parser責務
HTMLから以下を抽出する。
* タイトル
* URL
* 投稿日時

---

## 12.2 HTML変更耐性

selectorを parser.rs に集中管理する。

HTML変更時の修正範囲を限定する。

---

# 13. Discord通知設計

## 13.1 通知形式
```text
証券コード/銘柄名
公開時刻:2026-05-21 10:30
○○の株主優待変更
<https://...>
```

---
## 13.2 通知失敗時
* retry実施
* state更新しない
* 次回監視で再通知可能状態を維持

---
# 14. ログ設計
## 14.1 ログレベル
* INFO
* WARN
* ERROR

---
## 14.2 ログ方針
必ず以下を記録する。
* 起動
* 更新有無
* 通知成功
* 通知失敗
* parse失敗
* state更新

---
# 15. エラー設計
## 15.1 方針
panicを避け、独自Error型で管理する。

---
## 15.2 Error分類
```rust
enum AppError {
    Http,
    Parse,
    Discord,
    Storage,
    Config,
}
```

---

# 16. CLI設計

## 16.1 initialize
初回状態作成。
```bash id="vb9h1h"
cargo run -- initialize
```

---

## 16.2 monitor
通常監視。
```bash id="jlwm3i"
cargo run -- monitor
```

---

# 17. 使用ライブラリ候補

| 用途     | ライブラリ                 |
| ------ | --------------------- |
| HTTP   | reqwest               |
| HTML解析 | scraper               |
| JSON   | serde                 |
| ログ     | tracing               |
| Error  | thiserror             |
| 日時     | chrono                |
| retry  | backon / custom retry |

---

---

# 19. 将来拡張
## 想定拡張
* 複数サイト監視
* SQLite移行
* Slack通知
* Docker化
* GitHub Actions定期実行
* RSS対応
* Web UI

