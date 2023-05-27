# csv-to-sql

CSVを読み取り、TSVやSQLに変換するCLIツール

# 要件

- Shift-JISのプレーンテキストファイルを読み取り、UTF-8に変換して出力する
- UTF-8のCSVファイルを読み取り、UTF-8のTSVに変換して出力する
- UTF-8のTSVファイルを読み取り、select文で構成されたSQLファイルを出力する
- UTF-8のTSVファイルを読み取り、insert文で構成されたSQLファイルを出力する
- UTF-8のTSVファイルを読み取り、各カラムの最大バイト長などをTSV形式で出力する
