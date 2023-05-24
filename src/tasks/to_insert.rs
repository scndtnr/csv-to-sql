use anyhow::{Context, Result};
use csv::StringRecord;
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

pub(crate) fn to_insert(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!(
        "[to_insert] Conversion Start. Input file: {}",
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
        let record = result?;
        records.push(record);
    }

    // HashMapのベクタをSQLのinsert文に変換する
    let table_name = input_path.file_name().unwrap().to_string_lossy();
    let output_text = records_to_insert_query(&table_name, &headers, records);

    // insert文テキストを出力先SQLファイルに書き込む
    let file_name = format!(
        "insert_{}",
        output_path.file_name().unwrap().to_string_lossy()
    );
    let output_path = output_path.with_file_name(file_name);
    let output_path = output_path.with_extension("sql");
    let mut output_file = fs::File::create(&output_path).with_context(|| {
        format!(
            "Fail to create output file: {}",
            output_path.to_string_lossy()
        )
    })?;
    output_file.write_all(output_text.as_bytes())?;

    println!(
        "[to_insert] Conversion End. Output file: {}",
        output_path.to_string_lossy()
    );

    Ok(())
}

/// 複数行のinsert文を作成する
/// 出力イメージ：
///     insert into input.tsv
///         (col1, col2, col3)
///     values
///         ('a', null, 'c'),
///         ('a', 'b', null)
///     ;
fn records_to_insert_query(
    table_name: &str,
    headers: &StringRecord,
    records: Vec<HashMap<String, String>>,
) -> String {
    let insert_values_vec: Vec<String> = records
        .into_iter()
        .map(|record| to_insert_value(headers, record))
        .collect();
    let columns = headers.iter().collect::<Vec<&str>>().join(", ");
    let values = insert_values_vec.join(",\n");
    format!(
        "insert into {}\n    ({})\nvalues\n{}\n;",
        table_name, columns, values
    )
}

/// 複数行のinsert文における values部分の1行を作成する
/// 出力イメージ:
///     ('a', null, 'c')
fn to_insert_value(headers: &StringRecord, record: HashMap<String, String>) -> String {
    let insert_value_vec: Vec<String> = headers
        .iter()
        .map(|header| {
            let cell_data = record
                .get(header)
                .with_context(|| format!("Fail to get cell data by column name: {}", header))
                .unwrap();
            super::convert_to_null_string(cell_data)
        })
        .collect();
    let insert_value = insert_value_vec.join(", ");
    format!("    ({})", insert_value)
}
