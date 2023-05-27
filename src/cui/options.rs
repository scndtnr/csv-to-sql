use clap::{Parser, ValueEnum};

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
    #[clap(value_enum, help = "変換タスクを指定する")]
    pub(super) task: Task,
    #[clap(help = "入力ファイルを配置したディレクトリのpathを指定する")]
    pub(super) src: String,
    #[clap(help = "出力ファイルを配置するディレクトリのpathを指定する")]
    pub(super) dst: String,
}

impl Opts {
    pub(super) fn src(&self) -> &str {
        &self.src
    }
    pub(super) fn dst(&self) -> &str {
        &self.dst
    }
    pub(super) fn task(&self) -> &Task {
        &self.task
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum Task {
    /// Shift-JIS のファイルを受け取り、UTF-8に変換して出力する
    ToUtf8,
    /// UTF-8のCSVを受け取り、TSVに変換して出力する
    ToTsv,
    /// UTF-8のTSVを受け取り、SQLのselect文に変換して出力する
    ToSelect,
    /// UTF-8のTSVを受け取り、SQLのinsert文に変換して出力する
    ToInsert,
    /// UTF-8のTSVを受け取り、TSV形式のテーブル定義に変換して出力する
    ToDefinition,
}
