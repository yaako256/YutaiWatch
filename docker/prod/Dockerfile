# docker/prod/Dockerfile
# 本番用DockerFile
# =========================
# Build Stage
# =========================
FROM rust:1.88 AS builder

WORKDIR /app

# 開発用からコピー
# Dockerbuildを高速にするため、面倒な手順を取る。
# 依存だけ先にコピー
COPY Cargo.toml Cargo.lock ./

# ダミーmain
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# 依存ビルドキャッシュ
RUN cargo build --release

# 本物を上書き
COPY src ./src

# 本番ビルド
RUN cargo build --release

# =========================
# Runtime Stage
# =========================
FROM alpine:3.22

RUN apk add --no-cache \
#    bash \
    tzdata \
    curl

# timezone設定
ENV TZ=Asia/Tokyo

RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
    && echo $TZ > /etc/timezone

# supercronic
RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
      ARCH="amd64"; \
    elif [ "$ARCH" = "aarch64" ]; then \
      ARCH="arm64"; \
    fi && \
    curl -fsSLO https://github.com/aptible/supercronic/releases/latest/download/supercronic-linux-${ARCH} && \
    mv supercronic-linux-${ARCH} /usr/local/bin/supercronic && \
    chmod +x /usr/local/bin/supercronic

WORKDIR /app

COPY --from=builder /app/target/release/yutai_watch /app/yutai_watch

COPY docker/prod/crontab /app/crontab
COPY docker/prod/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]