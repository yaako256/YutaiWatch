# RustのDocker環境の構築メモ
Rustだけ(フロントエンドなし)で開発するときのDocker関連の設定を備忘録として示す。

## compose.yaml
以下のようにcompose.yamlを書く。  
サービスが一つでもdefaultでネットワークが作られるので、それも示す(任意)。
```yaml
# compose.yaml
services:
  # Rust
  app:
    build: .
    container_name: yutai_watch
    restart: unless-stopped
    networks:
      - default
```
* `build`はどこにある**DockerFile*を使ってbuildするか。フロントエンドと分けないため`.`で良い