use anyhow::Result;
use std::{path::PathBuf, str::FromStr};

mod controller;
mod to_select;
mod to_table_definition;
mod to_tsv;
mod to_utf8;

use std::fs;

pub(super) use controller::{convert_files, make_start_and_end_paths};

/// 文字列を `PathBuf` に変換する
pub(super) fn convert_to_path(path_str: &str) -> PathBuf {
    PathBuf::from_str(path_str).expect("Fail to &str to PathBuf.")
}

/// 指定されたファイル、あるいはディレクトリ直下のファイル一覧を返す
pub(super) fn read_input_files(src_path: PathBuf) -> Result<Vec<PathBuf>> {
    // 指定pathがファイルなら単要素ベクタを返す
    if src_path.is_file() {
        return Ok(vec![src_path]);
    }

    // 指定path配下のファイル一覧を返す
    Ok(fs::read_dir(src_path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect())
}
