"""
scraper/main.py
スクレイピングをし、jsonを出力する関数
"""

import json
from datetime import datetime, timezone, timedelta


def main():
    jst = timezone(timedelta(hours=9))

    output = {
        "fetched_at": datetime.now(jst).isoformat(),
        "items": [
            {
                "item_key": "20260525_001",
                "ticker_symbol": "1234",
                "ticker_name": "株式会社サンプル",
                "published_at": "2026-05-25T09:00:00+09:00",
                "title": "株主優待変更のお知らせ",
                "url": "https://example.com/item1",
            },
            {
                "item_key": "20260525_002",
                "ticker_symbol": "5678",
                "ticker_name": "テストホールディングス",
                "published_at": "2026-05-25T10:30:00+09:00",
                "title": "優待新設のお知らせ",
                "url": "https://example.com/item2",
            }
        ]
    }

    print(json.dumps(output, ensure_ascii=False))

if __name__ == "__main__":
    main()