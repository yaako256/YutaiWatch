"""
scraper/元main.py

playwright でページを取得し、株主優待ニュース一覧をパースして JSON を stdout に出力する。
エラー発生時は stderr に出力して sys.exit(1) で終了する。

元mainはただ取得するだけとなっている。
mainの方ではtf-playwright-stealthやリトライ処理、python側ジッターを追加する。
"""

# ── 標準ライブラリ ────────────────────────────────────────────────
import json
import sys
from datetime import datetime, timezone, timedelta

# ── サードパーティ ────────────────────────────────────────────────
from bs4 import BeautifulSoup
from playwright.sync_api import sync_playwright, TimeoutError as PlaywrightTimeoutError

# ── 定数 ─────────────────────────────────────────────────────────
TARGET_URL   = "https://www.invest-jp.net/yuutai/news"
TABLE_ID     = "table-news"
JST          = timezone(timedelta(hours=9))
PAGE_TIMEOUT = 30_000  # ms


# ── 例外 ─────────────────────────────────────────────────────────
class ScrapingError(Exception):
    """スクレイピング処理で発生した致命的なエラー"""


# ── スクレイピング ────────────────────────────────────────────────
def scrape(url: str) -> str:
    """
    Playwright でページを開き、HTML 文字列を返す。
    ネットワーク障害・タイムアウト・テーブル未検出の場合は ScrapingError を送出する。
    """
    try:
        with sync_playwright() as pw:
            browser = pw.chromium.launch(headless=True)
            try:
                page = browser.new_page()
                page.goto(url, timeout=PAGE_TIMEOUT, wait_until="domcontentloaded")
                # テーブルが DOM に現れるまで待機
                page.wait_for_selector(f"#{TABLE_ID}", timeout=PAGE_TIMEOUT)
                html = page.content()
            finally:
                browser.close()
    except PlaywrightTimeoutError as e:
        raise ScrapingError(f"タイムアウト: {e}") from e
    except Exception as e:
        raise ScrapingError(f"ページ取得に失敗しました: {e}") from e

    return html


# ── パース ────────────────────────────────────────────────────────
def _cell_text(td) -> str:
    """<td> の表示テキストを空白除去して返す"""
    return td.get_text(strip=True)


def _cell_href(td) -> str:
    """<td> 内の最初の <a> の href を返す。なければ空文字"""
    a = td.find("a", href=True)
    return a["href"] if a else ""


def parse_rows(html: str) -> list[dict]:
    """
    HTML から #table-news の tbody 行をパースし、dict のリストを返す。
    テーブルが見つからない場合は ScrapingError を送出する。

    各 dict のキー:
        ticker_symbol : str  証券コード
        ticker_name   : str  銘柄名
        published_at  : str  公開日時文字列（サイト掲載値をそのまま使用）
        title         : str  ニュースタイトル
        url           : str  記事 URL
    """
    soup = BeautifulSoup(html, "html.parser")

    table = soup.find("table", id=TABLE_ID)
    if table is None:
        raise ScrapingError(f"テーブル #{TABLE_ID} が見つかりませんでした")

    tbody = table.find("tbody")
    if tbody is None:
        raise ScrapingError(f"テーブル #{TABLE_ID} に <tbody> が存在しません")

    rows: list[dict] = []
    for tr in tbody.find_all("tr"):
        tds = tr.find_all("td")
        # 列数が足りない行はスキップ（ヘッダー混入・空行対策）
        if len(tds) < 4:
            continue

        ticker_symbol = _cell_text(tds[0])
        ticker_name   = _cell_text(tds[1])
        published_at  = _cell_text(tds[2])
        title         = _cell_text(tds[3])
        url           = _cell_href(tds[3])

        # 必須フィールドがすべて空の行はスキップ
        if not any([ticker_symbol, ticker_name, title]):
            continue

        rows.append({
            "ticker_symbol": ticker_symbol,
            "ticker_name":   ticker_name,
            "published_at":  published_at,
            "title":         title,
            "url":           url,
        })

    return rows


# ── 出力構造の組み立て ────────────────────────────────────────────
def build_output(rows: list[dict]) -> dict:
    """
    パース済み行リストから最終 JSON 構造を組み立てる。

    item_key は "YYYYMMDD_NNN" 形式（取得日 + 連番）。
    """
    today_str = datetime.now(JST).strftime("%Y%m%d")

    items = [
        {
            "item_key":      f"{today_str}_{idx:03d}",
            "ticker_symbol": row["ticker_symbol"],
            "ticker_name":   row["ticker_name"],
            "published_at":  row["published_at"],
            "title":         row["title"],
            "url":           row["url"],
        }
        for idx, row in enumerate(rows, start=1)
    ]

    return {
        "fetched_at": datetime.now(JST).isoformat(),
        "items":      items,
    }


# ── エントリポイント ──────────────────────────────────────────────
def main() -> None:
    try:
        html   = scrape(TARGET_URL)
        rows   = parse_rows(html)
        output = build_output(rows)
    except ScrapingError as e:
        print(f"[ERROR] {e}", file=sys.stderr)
        sys.exit(1)

    # Rust 側が stdout を読む前提 → indent なしでコンパクト出力
    print(json.dumps(output, ensure_ascii=False))


if __name__ == "__main__":
    main()