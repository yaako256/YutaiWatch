"""
scraper/debug.py
デバッグ用で内容を直接出力するpython
"""
# 標準ライブラリ
# Json用
import json
# 現在時刻取得用
from datetime import datetime, timezone, timedelta


def main():
    # 日本時間を取得
    jst = timezone(timedelta(hours=9))

    # items保存用
    items = []

    # スクレイピング結果を想定
    scraped_data = [
        {
            "ticker_symbol": "1234",
            "ticker_name": "株式会社サンプル",
            "published_at": "2026-05-25T09:00:00+09:00",
            "title": "株主優待変更のお知らせ",
            "url": "https://example.com/item1",
        },
        {
            "ticker_symbol": "5678",
            "ticker_name": "テストホールディングス",
            "published_at": "2026-05-25T10:30:00+09:00",
            "title": "優待新設のお知らせ",
            "url": "https://example.com/item2",
        },
    ]

    # ループで追加
    for idx, row in enumerate(scraped_data, start=1):
        item = {
            "item_key": f"20260525_{idx:03d}",
            "ticker_symbol": row["ticker_symbol"],
            "ticker_name": row["ticker_name"],
            "published_at": row["published_at"],
            "title": row["title"],
            "url": row["url"],
        }
        # itemsに追加
        items.append(item)

    # 最終JSON
    output = {
        "fetched_at": datetime.now(jst).isoformat(),
        "items": items,
    }

    print(json.dumps(output, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()