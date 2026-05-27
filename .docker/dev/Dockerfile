# .docker/dev/Dockerfile
# 開発用DockerFile

FROM rust:1.88

# Python
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip

# Pythonライブラリ
RUN pip3 install --break-system-packages \
    playwright \
    tf-playwright-stealth \
    beautifulsoup4

# Playwright + Chromium
RUN python3 -m playwright install --with-deps chromium

# Rust開発ツール
RUN cargo install cargo-watch

WORKDIR /app
