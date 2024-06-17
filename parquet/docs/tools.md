# Tools for working with Parquet Files

Use the binaries shipped with rust parquet - [parquet/src/bin](https://github.com/apache/arrow-rs/tree/master/parquet/src/bin).

Installation:

```sh
cargo install parquet --features=cli
```

File in scope:
```sh
file="test-data/nation.parquet"
```

## CSV to Parquet

```sh
parquet-fromcsv --schema  test-data/nation.schema \
    --input-file test-data/nation.csv \
    --output-file test-data/nation.parquet
```

## Read

```sh
parquet-read $file --json | jq
```

## Schema

Print the schema and metadata.

```sh
parquet-schema $file

parquet-schema $file --verbose
```

## Index

Prints the page index.

```sh
parquet-index $file nation_key

parquet-index $file name
```

## Layout

Prints the physical layout.


```sh
parquet-layout $file
```