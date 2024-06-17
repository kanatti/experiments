# Experiments with Parquet

References:
- https://parquet.apache.org/docs/
- https://github.com/apache/parquet-format
- https://github.com/jcrobak/parquet-python
- https://github.com/dask/fastparquet

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
                                                             Key Value Metadata
┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ key          ┃ value                                                                                                                     ┃
┡━━━━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┩
│ ARROW:schema │ /////xgBAAAQAAAAAAAKAAwACgAJAAQACgAAABAAAAAAAQQACAAIAAAABAAIAAAABAAAAAQAAACsAAAAbAAAADQAAAAEAAAAdP///xQAAAAMAAAAAAABBQwAA │
│              │ AAAAAAAoP///wsAAABjb21tZW50X2NvbACg////EAAAABgAAAAAAAECFAAAAJD///8gAAAAAAAAAQAAAAAKAAAAcmVnaW9uX2tleQAA1P///xgAAAAMAAAAAA │
│              │ ABBRAAAAAAAAAABAAEAAQAAAAEAAAAbmFtZQAAAAAQABQAEAAOAA8ABAAAAAgAEAAAABgAAAAgAAAAAAABAhwAAAAIAAwABAALAAgAAAAgAAAAAAAAAQAAAAA │
│              │ KAAAAbmF0aW9uX2tleQAA                                                                                                     │
└──────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
          Row Group 0
┌───────────────────────┬──────┐
│ num_rows              │ 125  │
│ file_offset           │ 4    │
│ total_byte_size       │ 3041 │
│ total_compressed_size │ 2174 │
│ ordinal               │ 0    │
└───────────────────────┴──────┘
                         Column - ['nation_key']
┌─────────────────────────┬──────────────────────────────────────────────┐
│ file_offset             │ 251                                          │
│ type                    │ INT32                                        │
│ encodings               │ [0, 3, 8]                                    │
│ path_in_schema          │ ['nation_key']                               │
│ codec                   │ SNAPPY                                       │
│ num_values              │ 125                                          │
│ total_uncompressed_size │ 241                                          │
│ total_compressed_size   │ 247                                          │
│ key_value_metadata      │ None                                         │
│ data_page_offset        │ 123                                          │
│ index_page_offset       │ None                                         │
│ dictionary_page_offset  │ 4                                            │
│ statistics              │ ┌────────────────────┬─────────────────────┐ │
│                         │ │ max                │ b'\x18\x00\x00\x00' │ │
│                         │ │ min                │ b'\x00\x00\x00\x00' │ │
│                         │ │ null_count         │ None                │ │
│                         │ │ distinct_count     │ None                │ │
│                         │ │ max_value          │ b'\x18\x00\x00\x00' │ │
│                         │ │ min_value          │ b'\x00\x00\x00\x00' │ │
│                         │ │ is_max_value_exact │ True                │ │
│                         │ │ is_min_value_exact │ True                │ │
│                         │ └────────────────────┴─────────────────────┘ │
│ encoding_stats          │ None                                         │
│ bloom_filter_offset     │ None                                         │
│ bloom_filter_length     │ None                                         │
│ size_statistics         │ None                                         │
└─────────────────────────┴──────────────────────────────────────────────┘
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
                         Column - ['region_key']
┌─────────────────────────┬──────────────────────────────────────────────┐
│ file_offset             │ 896                                          │
│ type                    │ INT32                                        │
│ encodings               │ [0, 3, 8]                                    │
│ path_in_schema          │ ['region_key']                               │
│ codec                   │ SNAPPY                                       │
│ num_values              │ 125                                          │
│ total_uncompressed_size │ 125                                          │
│ total_compressed_size   │ 129                                          │
│ key_value_metadata      │ None                                         │
│ data_page_offset        │ 803                                          │
│ index_page_offset       │ None                                         │
│ dictionary_page_offset  │ 767                                          │
│ statistics              │ ┌────────────────────┬─────────────────────┐ │
│                         │ │ max                │ b'\x04\x00\x00\x00' │ │
│                         │ │ min                │ b'\x00\x00\x00\x00' │ │
│                         │ │ null_count         │ None                │ │
│                         │ │ distinct_count     │ None                │ │
│                         │ │ max_value          │ b'\x04\x00\x00\x00' │ │
│                         │ │ min_value          │ b'\x00\x00\x00\x00' │ │
│                         │ │ is_max_value_exact │ True                │ │
│                         │ │ is_min_value_exact │ True                │ │
│                         │ └────────────────────┴─────────────────────┘ │
│ encoding_stats          │ None                                         │
│ bloom_filter_offset     │ None                                         │
│ bloom_filter_length     │ None                                         │
│ size_statistics         │ None                                         │
└─────────────────────────┴──────────────────────────────────────────────┘
                                                          Column - ['comment_col']
┌─────────────────────────┬────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ file_offset             │ 2360                                                                                                           │
│ type                    │ BYTE_ARRAY                                                                                                     │
│ encodings               │ [0, 3, 8]                                                                                                      │
│ path_in_schema          │ ['comment_col']                                                                                                │
│ codec                   │ SNAPPY                                                                                                         │
│ num_values              │ 125                                                                                                            │
│ total_uncompressed_size │ 2251                                                                                                           │
│ total_compressed_size   │ 1400                                                                                                           │
│ key_value_metadata      │ None                                                                                                           │
│ data_page_offset        │ 2079                                                                                                           │
│ index_page_offset       │ None                                                                                                           │
│ dictionary_page_offset  │ 960                                                                                                            │
│ statistics              │ ┌────────────────────┬───────────────────────────────────────────────────────────────────────────────────────┐ │
│                         │ │ max                │ None                                                                                  │ │
│                         │ │ min                │ None                                                                                  │ │
│                         │ │ null_count         │ None                                                                                  │ │
│                         │ │ distinct_count     │ None                                                                                  │ │
│                         │ │ max_value          │ b'y final packages. slow foxes cajole quickly. quickly silent platelets breach ironic │ │
│                         │ │                    │ accounts. unusual pinto be'                                                           │ │
│                         │ │ min_value          │ b' haggle. carefully final deposits detect slyly agai'                                │ │
│                         │ │ is_max_value_exact │ True                                                                                  │ │
│                         │ │ is_min_value_exact │ True                                                                                  │ │
│                         │ └────────────────────┴───────────────────────────────────────────────────────────────────────────────────────┘ │
│ encoding_stats          │ None                                                                                                           │
│ bloom_filter_offset     │ None                                                                                                           │
│ bloom_filter_length     │ None                                                                                                           │
│ size_statistics         │ None                                                                                                           │
└─────────────────────────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘


column_orders: [ColumnOrder(TYPE_ORDER=TypeDefinedOrder()), ColumnOrder(TYPE_ORDER=TypeDefinedOrder()), ColumnOrder(TYPE_ORDER=TypeDefinedOrder()), ColumnOrder(TYPE_ORDER=TypeDefinedOrder())]
encryption_algorithm: None
footer_signing_key_metadata: None
```


## Parquet Concepts

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

### Metadata

There are three types of metadata -  file metadata, column (chunk) metadata and page header metadata.

Metadata is covered in [docs/metadata.md](./docs/metadata.md)