どうやらtimerで定期実行より、
Rustに定期実行のコードも含めちゃって、それだけを実行してた方がコンテナが軽くなる！？！？
```
cron daemon
+
Rust app
```
より
```
Rust app only
```
かもしれない。
→やっぱ違うかも。