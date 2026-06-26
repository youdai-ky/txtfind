# txtfind

テキストファイル内の文字列を検索するシンプルなCLIツールです。

## 概要

`txtfind` は，現在の階層にあるファイルを一覧表示し，その中から選択した複数ファイルを対象に文字列検索を行うCLIツールです。

実行後に検索対象ファイルを番号で選択し，その後に検索文字列を入力します。検索結果は，デフォルトでファイル名・行番号・一致した行を表示します。

## 機能

- 現在の階層にあるファイル一覧の表示
- 隠しファイルを含む通常ファイルの表示
- 複数ファイルの選択
- 指定文字列の検索
- ファイル名・行番号・一致行の表示
- 大文字・小文字を区別しない検索
- 一致件数のみの表示

## 使い方

```text
txtfind [OPTIONS]
```

実行すると，現在の階層にあるファイル一覧が表示されます。  
検索対象にしたいファイルの番号を入力し，その後に検索文字列を入力します。

## オプション

```text
-i, --ignore-case     大文字・小文字を区別せずに検索する
-n, --line-number     行番号を表示する（現在はデフォルトで表示）
-c, --count           一致件数のみを表示する
-h, --help            ヘルプを表示する
-V, --version         バージョン情報を表示する
```

## 実行例

```bash
cargo run
```

表示例：

```text
Files in current directory:
  1. .gitignore
  2. Cargo.toml
  3. README.md
  4. log.txt

Select file numbers:
```

複数ファイルを指定する場合は，以下のように入力します。

```text
1,4
```

または，空白区切りでも指定できます。

```text
1 4
```

その後，検索文字列を入力します。

```text
Search pattern: error
```

出力形式は以下です。

```text
ファイル名:行番号: 一致した行
```

出力例：

```text
log.txt:1: error: first
log.txt:3: error: second
```

## 大文字・小文字を区別しない検索

```bash
cargo run -- --ignore-case
```

## 一致件数のみ表示

```bash
cargo run -- --count
```

## ビルド

```bash
cargo build
```

## テスト

```bash
cargo test
```

## 実際のCLIとして使う場合

リリースビルドを行います。

```bash
cargo build --release
```

実行ファイルは以下に作成されます。

```text
target/release/txtfind
```

直接実行する場合：

```bash
./target/release/txtfind
```

ローカルにインストールする場合：

```bash
cargo install --path .
```

インストール後は，以下のように実行できます。

```bash
txtfind
txtfind --ignore-case
txtfind --count
```

## 開発方針

このプロジェクトでは，読みやすく，テストしやすいプログラムを目指します。  
処理は関数ごとに分割し，ファイル一覧取得，入力処理，検索処理，出力整形を分けています。

## プロジェクト情報

- ツール名: txtfind
- 開発言語: Rust
- バージョン: 0.1.0
