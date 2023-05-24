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

/// 空文字列等を`null`表記に変換する
fn convert_to_null_string(s: &str) -> String {
    match s {
        s if s.is_empty() => "null".to_string(),
        s if s == "\"\"" => "null".to_string(),
        s if s == "''" => "null".to_string(),
        s if s == "null" => "null".to_string(),
        s => format!("'{}'", s),
    }
}
