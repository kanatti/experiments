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
