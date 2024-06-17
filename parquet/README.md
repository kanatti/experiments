# Experiments with Parquet

References:
- https://parquet.apache.org/docs/
- https://github.com/apache/parquet-format
- https://github.com/jcrobak/parquet-python
- https://github.com/dask/fastparquet
- https://github.com/apache/arrow/tree/main/go/parquet
- https://github.com/apache/arrow/tree/main/python/pyarrow/parquet
- https://github.com/julienledem/redelm/wiki/The-striping-and-assembly-algorithms-from-the-Dremel-paper

## Plan

This will be more of a hands-on learning instead of just reading through docs/code. Implementations are already available in python/rust, but we will try to re-implement those just to learn the format better.

1. Implement metadata utils in python
2. Implement reader in python (no optimizations)
3. Implement optimized reader in python
4. Implement writer in python
5. Rust implementation
6. Benchmarks and insights.
7. Diagramming and write blogs.


## Thrift bootstrapping

```sh
thrift -r --gen py --out src/parquet_py/ src/parquet_py/parquet.thrift
```

## Run read-example

```sh
❯ rye run read-example
                   Info
┌────────────┬───────────────────────────┐
│ version    │ 1                         │
│ num_rows   │ 125                       │
│ created_by │ parquet-rs version 52.0.0 │
└────────────┴───────────────────────────┘
                                      Schema Elements
┏━━━━━━━━━━━━━┳━━━━━━━━━━━━┳━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━┓
┃ name        ┃ type       ┃ type_length ┃ num_children ┃ repetition_type ┃ converted_type ┃
┡━━━━━━━━━━━━━╇━━━━━━━━━━━━╇━━━━━━━━━━━━━╇━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━┩
│ m           │ -          │ None        │ 4            │ -               │ -              │
│ nation_key  │ INT32      │ None        │ None         │ OPTIONAL        │ -              │
│ name        │ BYTE_ARRAY │ None        │ None         │ OPTIONAL        │ UTF8           │
│ region_key  │ INT32      │ None        │ None         │ OPTIONAL        │ -              │
│ comment_col │ BYTE_ARRAY │ None        │ None         │ OPTIONAL        │ UTF8           │
└─────────────┴────────────┴─────────────┴──────────────┴─────────────────┴────────────────┘
...
                        Column - ['name']
┌─────────────────────────┬─────────────────────────────────────┐
│ file_offset             │ 714                                 │
│ type                    │ BYTE_ARRAY                          │
│ encodings               │ [0, 3, 8]                           │
│ path_in_schema          │ ['name']                            │
│ codec                   │ SNAPPY                              │
│ num_values              │ 125                                 │
│ total_uncompressed_size │ 424                                 │
│ total_compressed_size   │ 398                                 │
│ key_value_metadata      │ None                                │
│ data_page_offset        │ 580                                 │
│ index_page_offset       │ None                                │
│ dictionary_page_offset  │ 316                                 │
│ statistics              │ ┌────────────────────┬────────────┐ │
│                         │ │ max                │ None       │ │
│                         │ │ min                │ None       │ │
│                         │ │ null_count         │ None       │ │
│                         │ │ distinct_count     │ None       │ │
│                         │ │ max_value          │ b'VIETNAM' │ │
│                         │ │ min_value          │ b'ALGERIA' │ │
│                         │ │ is_max_value_exact │ True       │ │
│                         │ │ is_min_value_exact │ True       │ │
│                         │ └────────────────────┴────────────┘ │
│ encoding_stats          │ None                                │
│ bloom_filter_offset     │ None                                │
│ bloom_filter_length     │ None                                │
│ size_statistics         │ None                                │
└─────────────────────────┴─────────────────────────────────────┘
...
```


## Parquet Concepts

- Metadata
    - FileMetadata, ColumnMetaData, PageHeader
    - Metadata is covered in [docs/metadata.md](./docs/metadata.md)
- RowGroup
- Path
- Offset Index
- Column Index
- Bloom Filter
- Pages
- Encodings
    - Plain
    - RLE
    - RLE Dict
- Compression
