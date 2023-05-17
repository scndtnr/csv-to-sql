use clap::{Parser, Subcommand};

/// コマンドライン引数のパース用構造体
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "CSV to SQL",
    version = "0.1.0",
    author = "zumi",
    about = "A CLI tool that reads CSV files and converts them to TSV or SQL."
)]
#[clap(propagate_version = true)]
pub(crate) struct Opts {
    #[clap(help = "入力ファイルを配置したディレクトリのpathを指定する")]
    pub(super) src: String,
    #[clap(help = "出力ファイルを配置するディレクトリのpathを指定する")]
    pub(super) dst: String,

    #[clap(subcommand)]
    pub(super) tasks: Tasks,
}

impl Opts {
    pub(super) fn src(&self) -> &str {
        &self.src
    }
    pub(super) fn dst(&self) -> &str {
        &self.dst
    }
    pub(super) fn tasks(&self) -> &Tasks {
        &self.tasks
    }
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Tasks {
    /// Shift-JIS のファイルを受け取り、UTF-8に変換して出力する
    Utf8,
    /// UTF-8のCSVを受け取り、TSVに変換して出力する
    Tsv,
    /// UTF-8のTSVを受け取り、SQLのselect文に変換して出力する
    Query,
    /// UTF-8のTSVを受け取り、TSV形式のテーブル定義に変換して出力する
    Definition,
}
