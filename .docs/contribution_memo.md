
# contribution問題解決の備忘録
## 問題
githubでリポジトリをのぞいたらcontributionに自分ではない人がいる。

---

## 原因
```
・自宅サーバ側で
git config --global user.name
git config --global user.email
を確認してみたら
aaa
aaa@aaa
となっており、仮のままだったため、検索？ができなかった。
(旧環境のVM環境だとyaakoとyaako@yaakoだがなぜか反映されていた)
```
## 解決方法
```
git config --global user.name "新ユーザ名"
git config --global user.email "新メアド"
って感じで、ちゃんとしたやつを入れたら解決
今後のコミットは正常になる
```

---
# 履歴を変更
しかし、履歴も自分にしたい。  
以下のコマンドで履歴を頑張って直したら今までのも一応自分判定させれた


## 実際に使用したコマンド

### 状況
- ubuntu server (linux)で実行している
- 事前にfilter-repoを入れる必要あり
- フォルダ構造はworkSpace/{リポジトリ名}となっている
- 昔の情報は {旧ユーザ名}および{旧メアド}
- 今の情報は{新ユーザ名}および{新メアド}
```bash
# 使用コマンド
$ cd ~/workSpace
$ git clone https://github.com/{新ユーザ名}/{リポジトリ名}.git {リポジトリ名}-clean
$ cd {リポジトリ名}-clean
$ git clone --bare . ../{リポジトリ名}-clean.git.backup
$ git filter-repo --mailmap <(cat << 'EOF'
{新ユーザ名} <{新メアド}> {旧ユーザ名} <{旧メアド}>
EOF
) --force
$ git remote remove origin
$ git remote add origin https://github.com/{新ユーザ名}/{リポジトリ名}.git
$ git remote -v
$ git push --force origin main
$ git log --all --pretty=format:"%an <%ae>" | sort | uniq
$ sudo rm -rf {リポジトリ名}-clean {リポジトリ名}-clean.git.backup
```
```bash
# 実行用に`$ `をなくした版
cd ~/workSpace
git clone https://github.com/{新ユーザ名}/{リポジトリ名}.git {リポジトリ名}-clean
cd {リポジトリ名}-clean
git clone --bare . ../{リポジトリ名}-clean.git.backup
git filter-repo --mailmap <(cat << 'EOF'
{新ユーザ名} <{新メアド}> {旧ユーザ名} <{旧メアド}>
EOF
) --force
git remote remove origin
git remote add origin https://github.com/{新ユーザ名}/{リポジトリ名}.git
git remote -v
git push --force origin main
git log --all --pretty=format:"%an <%ae>" | sort | uniq
cd ~/workSpace
sudo rm -rf {リポジトリ名}-clean {リポジトリ名}-clean.git.backup
```

---
# もしかしたら
originと同期がされてないとかでエラーが起きるかもしれない。  
次のコマンドで解決する
```bash
# pull
git pull --rebase origin main

# 確認
git log --oneline -5
git status
```