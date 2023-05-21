use anyhow::{Context, Result};
use csv::StringRecord;
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

pub(crate) fn to_query(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!(
        "[to_query] Conversion Start. Input file: {}",
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

    // HashMapのベクタをSQLのselect文（from句なし）に変換する
    let output_text = records_to_select_query(&headers, records);

    // select文テキストを出力先SQLファイルに書き込む
    let output_path = output_path.with_extension("sql");
    let mut output_file = fs::File::create(&output_path).with_context(|| {
        format!(
            "Fail to create output file: {}",
            output_path.to_string_lossy()
        )
    })?;
    output_file.write_all(output_text.as_bytes())?;

    println!(
        "[to_query] Conversion End. Output file: {}",
        output_path.to_string_lossy()
    );

    Ok(())
}

/// 複数のselect文をunion allで繋いだselect文を作成する
/// 出力イメージ：
///     select 'a1' as "X", 'b1' as "Y"
///     union all
///     select 'a2' as "X", 'b2' as "Y"
fn records_to_select_query(
    headers: &StringRecord,
    records: Vec<HashMap<String, String>>,
) -> String {
    let select_query_vec: Vec<String> = records
        .into_iter()
        .map(|record| to_select_clause(headers, record))
        .collect();
    select_query_vec.join("\nunion all\n")
}

/// select文を1文作成する
/// 出力イメージ:
///     select 'a' as "X", 'b' as "Y"
fn to_select_clause(headers: &StringRecord, record: HashMap<String, String>) -> String {
    let select_clause_vec: Vec<String> = headers
        .iter()
        .map(|header| {
            let cell_data = record
                .get(header)
                .with_context(|| format!("Fail to get cell data by column name: {}", header))
                .unwrap();
            format!("'{}' as \"{}\"", cell_data, header)
        })
        .collect();
    let select_clause = select_clause_vec.join(", ");
    format!("select {}", select_clause)
}
