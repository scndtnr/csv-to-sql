use anyhow::Result;
use std::{path::PathBuf, str::FromStr};

mod controller;
mod to_query;
mod to_table_definition;
mod to_tsv;
mod to_utf8;

pub(super) use controller::{convert_files, make_start_and_end_paths};

/// 文字列を `PathBuf` に変換する
pub(super) fn convert_to_path(path_str: &str) -> PathBuf {
    PathBuf::from_str(path_str).expect("Fail to &str to PathBuf.")
}

/// 指定されたディレクトリ直下のファイル一覧を取得する
pub(super) fn read_input_files(src_dir: PathBuf) -> Result<Vec<PathBuf>> {
    todo!("impl read_input_files")
}
