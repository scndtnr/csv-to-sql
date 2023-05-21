use anyhow::Result;
use std::path::PathBuf;

pub(crate) fn to_tsv(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!(
        "[to_tsv] Conversion Start. Input file: {}",
        input_path.to_string_lossy()
    );

    // 拡張子をcsvからtsvに変更する
    let output_path = output_path.with_extension("tsv");

    // csv用readerインスタンスを作成する
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(&input_path)?;
    // tsv用writerインスタンスを作成する
    let mut tsv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(&output_path)?;

    // csvを読み取りtsvをファイル出力する
    tsv_writer.write_record(csv_reader.headers()?)?;
    for result in csv_reader.records() {
        let record = result?;
        tsv_writer.write_record(&record)?;
        println!("{:?}", record);
    }

    println!(
        "[to_tsv] Conversion End. Output file: {}",
        output_path.to_string_lossy()
    );

    Ok(())
}
