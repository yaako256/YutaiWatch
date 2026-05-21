---
---

# RustのDocker環境の構築メモ
Rustだけ(フロントエンドなし)で開発するときのDocker関連の設定を備忘録として示す。

## ファイル構造
```text
YutaiWatch/
├── .dockerignore
├── .env
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── compose.yaml
├── generate_tree_ver2.py
├── Makefile
├── YAAKO_README.md
├── src/
│   └── main.rs
└── docker/
    ├── dev/
    │   └── Dockerfile
    └── prod/
        ├── crontab
        ├── Dockerfile
        └── entrypoint.sh
```

## メモ
`compose.yaml`で開発用コンテナと本番用コンテナを分けて定義。
`docker/`でその他の定義をしている。
`entrypoint.sh`でtimer機能を起動。詳細設定を`crontab`で行う。
`cargo init .`で現在のディレクトリをprojectにしたが、ファイル名はスネークケース統一にするため、Cargo.tomlを編集。
