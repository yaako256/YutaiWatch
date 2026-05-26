# Makefile
# PHONY: ファイルではないという指定(ファイルは更新されていないと実行されない): 命令である
.PHONY: run dev prod deploy down logs devlogs devshell build reset

# テスト実行
run:
	cargo run -p app

# 開発起動
dev:
	docker compose up yutai_watch_dev

# 本番起動
prod:
	docker compose up -d yutai_watch

# 完全本番デプロイ
# - dev停止
# - release build
# - container再作成
deploy:
	docker compose stop yutai_watch_dev
	docker compose rm -f yutai_watch_dev
	docker compose up -d --build --force-recreate yutai_watch

# 停止
down:
	docker compose down

# build
# ちゃんとbuildが通るか確認に使える
build:
	docker compose build

# 本番ログ
logs:
	docker compose logs -f yutai_watch_dev

# 開発ログ
devlogs:
	docker compose logs -f yutai_watch_dev

# 開発コンテナへ入る
devshell:
	docker compose exec yutai_watch_dev /bin/sh

# 完全リセット
reset:
	docker compose down --rmi all --volumes --remove-orphans