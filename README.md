# csv-to-sql

CSVを読み取り、TSVやSQLに変換するCLIツール

# 要件

- Shift-JISのプレーンテキストファイルを読み取り、UTF-8に変換して出力する
- UTF-8のCSVファイルを読み取り、UTF-8のTSVに変換して出力する
- UTF-8のTSVファイルを読み取り、select文で構成されたSQLファイルを出力する
- UTF-8のTSVファイルを読み取り、insert文で構成されたSQLファイルを出力する
- UTF-8のTSVファイルを読み取り、各カラムの最大バイト長などをTSV形式で出力する

# help

```
A CLI tool that reads CSV files and converts them to TSV or SQL.

Usage: csv-to-sql.exe <TASK> <SRC> <DST>

Arguments:
  <TASK>
          変換タスクを指定する

          Possible values:
          - to-utf8:       Shift-JIS のファイルを受け取り、UTF-8に変換して出力する
          - to-tsv:        UTF-8のCSVを受け取り、TSVに変換して出力する
          - to-select:     UTF-8のTSVを受け取り、SQLのselect文に変換して出力する
          - to-insert:     UTF-8のTSVを受け取り、SQLのinsert文に変換して出力する
          - to-definition: UTF-8のTSVを受け取り、TSV形式のテーブル定義に変換して出力する

  <SRC>
          入力ファイルを配置したディレクトリのpathを指定する

  <DST>
          出力ファイルを配置するディレクトリのpathを指定する

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```