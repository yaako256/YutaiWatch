# =========================
# Build Stage
# =========================
FROM rust:1.88 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY src ./src

RUN cargo build --release


# =========================
# Runtime Stage
# =========================
FROM python:3.12-slim

# 必要ライブラリ
RUN apt-get update && apt-get install -y \
    curl \
    tzdata \
    wget \
    ca-certificates \
    libnss3 \
    libatk1.0-0 \
    libatk-bridge2.0-0 \
    libcups2 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxrandr2 \
    libgbm1 \
    libgtk-3-0 \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

# timezone
ENV TZ=Asia/Tokyo

RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
    && echo $TZ > /etc/timezone

WORKDIR /app

# =========================
# Python dependencies
# =========================
COPY requirements.txt ./

RUN pip install --no-cache-dir -r requirements.txt

# Playwright（※requirementsに含めてない場合のみ）
RUN python -m playwright install --with-deps chromium

# =========================
# supercronic
# =========================
RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
      ARCH="amd64"; \
    elif [ "$ARCH" = "aarch64" ]; then \
      ARCH="arm64"; \
    fi && \
    curl -fsSLO https://github.com/aptible/supercronic/releases/latest/download/supercronic-linux-${ARCH} && \
    mv supercronic-linux-${ARCH} /usr/local/bin/supercronic && \
    chmod +x /usr/local/bin/supercronic

# =========================
# Rust binary
# =========================
COPY --from=builder /app/target/release/yutai_watch /app/yutai_watch

# scraper
COPY scraper ./scraper

COPY docker/prod/crontab /app/crontab
COPY docker/prod/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]