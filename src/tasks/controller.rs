use anyhow::Result;
use std::path::PathBuf;

use super::to_query::to_query;
use super::to_table_definition::to_table_definition;
use super::to_tsv::to_tsv;
use super::to_utf8::to_utf8;
use super::{convert_to_path, read_input_files};

use crate::cui::Tasks;

/// 入力ファイルパスと出力ファイルパスの一覧を生成する
pub(crate) fn make_start_and_end_paths(
    src: &str,
    dst: &str,
    tasks: &Tasks,
) -> Result<Vec<(PathBuf, PathBuf)>> {
    let src_dir = convert_to_path(src);
    let dst_dir = convert_to_path(dst);

    let output_dir_name = match tasks {
        Tasks::Utf8 => "utf8",
        Tasks::Tsv => "tsv",
        Tasks::Query => "query",
        Tasks::Definition => "table_definition",
    };
    let output_dir = dst_dir.join(output_dir_name);

    let paths: Vec<(PathBuf, PathBuf)> = read_input_files(src_dir)?
        .into_iter()
        .map(|input_file| {
            let input_name = input_file.file_name().expect("Fail to get file name.");
            let output_file = output_dir.join(input_name);
            (input_file, output_file)
        })
        .collect();
    Ok(paths)
}

/// 指定されたタスクに応じて変換処理を行う
pub(crate) fn convert_files(tasks: &Tasks, paths: Vec<(PathBuf, PathBuf)>) -> Result<()> {
    let convert_process = match tasks {
        Tasks::Utf8 => to_utf8,
        Tasks::Tsv => to_tsv,
        Tasks::Query => to_query,
        Tasks::Definition => to_table_definition,
    };
    paths.into_iter().for_each(|(input, output)| {
        convert_process(input, output).expect("Fail to convert process")
    });
    Ok(())
}
