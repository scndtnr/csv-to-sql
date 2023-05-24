use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

use super::to_query::to_query;
use super::to_table_definition::to_table_definition;
use super::to_tsv::to_tsv;
use super::to_utf8::to_utf8;
use super::{convert_to_path, read_input_files};

use crate::cui::Task;

/// 入力パス文字列と出力パス文字列のパス変換と存在チェック
fn convert_to_paths(src: &str, dst: &str) -> Result<(PathBuf, PathBuf)> {
    let src_path = convert_to_path(src);
    let dst_path = convert_to_path(dst);

    match (src_path.exists(), dst_path.exists()) {
        (true, false) => bail!("The dst path is not exists: {}", dst),
        (false, true) => bail!("The src path is not exists: {}", src),
        (false, false) => bail!("Both paths is not exists. src: {} , dst: {}", src, dst),
        (true, true) => Ok((src_path, dst_path)),
    }
}

/// パスを受け取り、ディレクトリなら返し、そうでなければエラーを返す
fn path_to_dir(path: PathBuf) -> Result<PathBuf> {
    if path.is_dir() {
        Ok(path)
    } else {
        bail!("The dst path is not directory: {}", path.to_string_lossy())
    }
}

/// 指定されたパスが存在しなかったら、ディレクトリを作成する
fn create_directory(dir_path: &Path) -> Result<()> {
    if !dir_path.exists() {
        println!("Create Directory: {}", dir_path.to_string_lossy());
        std::fs::create_dir_all(dir_path)?
    }
    Ok(())
}

/// 入力ファイルパスと出力ファイルパスの一覧を生成する
pub(crate) fn make_start_and_end_paths(
    src: &str,
    dst: &str,
    task: &Task,
) -> Result<Vec<(PathBuf, PathBuf)>> {
    // パス変換と存在チェック
    let (src_path, dst_path) = convert_to_paths(src, dst)?;
    let dst_dir = path_to_dir(dst_path)?;

    let output_dir_name = match task {
        Task::ToUtf8 => "utf8",
        Task::ToTsv => "tsv",
        Task::ToSelect => "sql/select",
        Task::ToInsert => "sql/insert",
        Task::ToDefinition => "table_definition",
    };
    let output_dir = dst_dir.join(output_dir_name);
    create_directory(&output_dir)?;

    let paths: Vec<(PathBuf, PathBuf)> = read_input_files(src_path)?
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
pub(crate) fn convert_files(task: &Task, paths: Vec<(PathBuf, PathBuf)>) -> Result<()> {
    let convert_process = match task {
        Task::ToUtf8 => to_utf8,
        Task::ToTsv => to_tsv,
        Task::ToSelect => to_query,
        Task::ToInsert => todo!(),
        Task::ToDefinition => to_table_definition,
    };
    paths.into_iter().for_each(|(input, output)| {
        convert_process(input, output).expect("Fail to convert process")
    });
    Ok(())
}
