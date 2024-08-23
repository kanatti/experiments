/// Metadata of a parquet file
pub struct FileMetaData {
    /// File Version
    pub version: i32,
    /// Number of rows in this file.
    pub num_rows: i64,
    /// Schema of columns
    pub schema: Vec<SchemaElement>,
    /// Metadata for the row groups in this file
    pub row_groups: Vec<RowGroupMetaData>,
    /// Optional key/value metadata
    pub key_value_metadata: Option<Vec<KeyValue>>,
    /// Application that created this file
    pub created_by: Option<String>,
}

/// Metadata for a row group
pub struct RowGroupMetaData {
    /// Number of rows in this row group
    pub num_rows: i64,
    /// Column chunks in this row group
    pub columns: Vec<ColumnChunkMetaData>,
    /// Total byte size of all uncompressed data in this row group
    pub total_byte_size: i64,
    /// Total byte size of all compressed data in this row group
    pub total_compressed_size: Option<i64>,
    /// Byte offset from beginning of file to first page in this row group
    pub file_offset: Option<i64>,
    /// Row group ordinal in the file
    pub ordinal: Option<i16>,
}

pub struct ColumnChunkMetaData {}

pub struct KeyValue {}

pub struct SchemaElement {}