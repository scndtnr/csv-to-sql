use std::collections::HashMap;

use anyhow::Context;
use csv::StringRecord;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(super) struct TableDefinitionRecord {
    sort_key: usize,
    column_name: String,
    data_type: String,
    data_length: usize,
}

impl TableDefinitionRecord {
    pub(super) fn new(sort_key: usize, column_name: &str, column_length: usize) -> Self {
        Self {
            sort_key,
            column_name: column_name.to_string(),
            data_type: "string".to_string(),
            data_length: column_length,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub(super) struct TableDefinitionRecords(pub(super) Vec<TableDefinitionRecord>);

impl TableDefinitionRecords {
    pub(super) fn new(headers: &StringRecord, records: Vec<HashMap<String, String>>) -> Self {
        let mut table_definition_records = Vec::new();
        for (sort_key, header) in headers.iter().enumerate() {
            let column_length: usize = records
                .iter()
                .map(|record| record.get(header))
                .filter_map(|cell| cell.map(|s| s.as_str()))
                .map(|data| data.len())
                .max()
                .with_context(|| format!("Fail to get max length of: {}", header))
                .unwrap();
            table_definition_records.push(TableDefinitionRecord::new(
                sort_key + 1,
                header,
                column_length,
            ))
        }
        Self(table_definition_records)
    }
}
