テキストファイル内の文字列を検索するシンプルなCLIツールです。

概要

txtfind は，現在の階層にあるファイルを一覧表示し，その中から選択した複数ファイルを対象に文字列検索を行うCLIツールです。

実行後に検索対象ファイルを番号で選択し，その後に検索文字列を入力します。検索結果は，デフォルトでファイル名・行番号・一致した行を表示します。

機能
現在の階層にあるファイル一覧の表示
隠しファイルを含む通常ファイルの表示
複数ファイルの選択
指定文字列の検索
ファイル名・行番号・一致行の表示
大文字・小文字を区別しない検索
一致件数のみの表示
シェル補完ファイルの生成
Dockerによる実行
Homebrewによるインストール
使い方
txtfind [OPTIONS]

実行すると，現在の階層にあるファイル一覧が表示されます。
検索対象にしたいファイルの番号を入力し，その後に検索文字列を入力します。

オプション
-i, --ignore-case     大文字・小文字を区別せずに検索する
-n, --line-number     行番号を表示する（現在はデフォルトで表示）
-c, --count           一致件数のみを表示する
    --completions     各シェル用の補完ファイルを生成する
-h, --help            ヘルプを表示する
-V, --version         バージョン情報を表示する
実行例
cargo run

表示例：

Files in current directory:
  1. .gitignore
  2. Cargo.toml
  3. README.md
  4. log.txt

Select file numbers:

複数ファイルを指定する場合は，以下のように入力します。

1,4

または，空白区切りでも指定できます。

1 4

その後，検索文字列を入力します。

Search pattern: error

出力形式は以下です。

ファイル名:行番号: 一致した行

出力例：

log.txt:1: error: first
log.txt:3: error: second
大文字・小文字を区別しない検索
cargo run -- --ignore-case
一致件数のみ表示
cargo run -- --count
補完機能

txtfind はシェル補完に対応しています。

以下のコマンドで，Bash，Zsh，Fish，PowerShell，Elvish 用の補完ファイルを生成できます。

cargo run -- --completions

生成された補完ファイルは completions/ 以下に保存されます。

completions/
├── bash/
├── elvish/
├── fish/
├── powershell/
└── zsh/

Homebrewでインストールした場合は，補完ファイルも一緒にインストールされます。

例えば，以下のように入力してTabキーを押すと，オプション候補が表示されます。

txtfind --

補完候補の例：

--help
--version
--ignore-case
--count
--line-number
--completions
ビルド
cargo build
テスト
cargo test
実際のCLIとして使う場合

リリースビルドを行います。

cargo build --release

実行ファイルは以下に作成されます。

target/release/txtfind

直接実行する場合：

./target/release/txtfind

ローカルにインストールする場合：

cargo install --path .

インストール後は，以下のように実行できます。

txtfind
txtfind --ignore-case
txtfind --count
Dockerでの実行方法

Dockerを使うことで，Rustの環境がない場合でも txtfind を実行できます。

Dockerイメージのビルド
docker build -t txtfind:latest -f Containerfile .
Dockerコンテナでの実行

macOS / Linux の場合：

docker run -it --rm -v "$PWD":/work txtfind:latest

Windows PowerShell の場合：

docker run -it --rm -v ${PWD}:/work txtfind:latest

実行すると，現在の階層にあるファイル一覧が表示されます。
番号で検索対象ファイルを選択し，その後に検索文字列を入力します。

Homebrewでのインストール

以下のコマンドで，txtfind をHomebrewからインストールできます。

brew tap youdai-ky/tap
brew install txtfind

または，以下のように一度に実行できます。

brew install youdai-ky/tap/txtfind

インストール後，以下のコマンドでバージョンを確認できます。

txtfind --version

実行例：

txtfind 0.1.0

Homebrewでインストールした場合は，通常のコマンドとして以下のように実行できます。

txtfind
開発方針

このプロジェクトでは，読みやすく，テストしやすいプログラムを目指します。
処理は関数ごとに分割し，ファイル一覧取得，入力処理，検索処理，出力整形，補完ファイル生成を分けています。

プロジェクト情報
ツール名: txtfind
開発言語: Rust
バージョン: 0.1.0
ライセンス: MIT
配布方法: GitHub Release，Homebrew，Docker