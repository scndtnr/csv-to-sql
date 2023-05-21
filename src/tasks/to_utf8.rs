use anyhow::{Context, Result};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

/// 入力ファイルを読み取り、Shift-JISからUTF-8に変換し、出力ファイルに書き込む
pub(crate) fn to_utf8(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!(
        "[to_utf8] Conversion Start. Input file: {}",
        input_path.to_string_lossy()
    );

    // Shift-JISの入力ファイルを読み取り、バイト列として取得する
    let mut input_buffer = Vec::new();
    let mut input_file = fs::File::open(&input_path)?;
    input_file.read_to_end(&mut input_buffer)?;

    // Shift-JIS のバイト列 から UTF-8 の文字列へと変換する
    let (output_buffer, _, _) = encoding_rs::SHIFT_JIS.decode(&input_buffer);
    let output_text = output_buffer.to_string();

    // 出力先ファイルにUTF-8としてテキストを書き込む
    let mut output_file = fs::File::create(&output_path).with_context(|| {
        format!(
            "Fail to create output file: {}",
            output_path.to_string_lossy()
        )
    })?;
    output_file.write_all(output_text.as_bytes())?;

    println!(
        "[to_utf8] Conversion End. Output file: {}",
        output_path.to_string_lossy()
    );

    Ok(())
}
