mod table_definition;

use self::table_definition::TableDefinitionRecords;

use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

pub(crate) fn to_table_definition(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!(
        "[to_table_definition] Analysis Start. Input file: {}",
        input_path.to_string_lossy()
    );

    // tsv用readerインスタンスを作成する
    let mut tsv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(&input_path)?;

    // tsvを読み取り、ヘッダとデータで構成されたHashMapのベクタにする
    let headers = tsv_reader.headers()?.to_owned();
    // イテレータにすると読み取れず結果が空になるため、forループを利用している
    let mut records = Vec::new();
    for result in tsv_reader.deserialize() {
        let record: HashMap<String, String> = result?;
        records.push(record);
    }

    // TableDefinitionRecords構造体に変換する
    let table_definition_records = TableDefinitionRecords::new(&headers, records);

    // TSV形式のテーブル定義を出力先ファイルに書き込む
    let output_path = output_path.with_extension("tsv");
    let mut tsv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(&output_path)?;

    for record in table_definition_records.0 {
        tsv_writer.serialize(record)?;
    }
    tsv_writer.flush()?;

    println!(
        "[to_table_definition] Analysis End. Output file: {}",
        output_path.to_string_lossy()
    );

    Ok(())
}
