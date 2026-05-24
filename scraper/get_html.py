from playwright.sync_api import sync_playwright


URL = "https://www.invest-jp.net/yuutai/news"


def fetch_html(url: str) -> str:
    with sync_playwright() as p:
        browser = p.chromium.launch(
            headless=True,
            args=["--no-sandbox"]  # Ubuntu Server向け
        )

        page = browser.new_page()

        page.goto(url, wait_until="networkidle")

        # Cloudflare待機
        page.wait_for_timeout(10000)

        html = page.content()

        browser.close()

        return html


def main():
    html = fetch_html(URL)

    print(html)

    with open("output.html", "w", encoding="utf-8") as f:
        f.write(html)

    print("保存完了")


if __name__ == "__main__":
    main()