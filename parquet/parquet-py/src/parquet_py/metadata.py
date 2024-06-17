from typing import List
from .parquet import ttypes
from rich.console import Console
from rich.table import Table

RICH_CONSOLE = Console()

def _resolve_enum(i, enum_class):
    return enum_class._VALUES_TO_NAMES.get(i, "-")

def _print_info(info: ttypes.FileMetaData):
    table = Table(title="Info", show_header=False)
    table.add_row("version", str(info.version))
    table.add_row("num_rows", str(info.num_rows))
    table.add_row("created_by", info.created_by)

    RICH_CONSOLE.print(table)

def _print_key_value_metadata(kvs: List[ttypes.KeyValue]):
    table = Table(title="Key Value Metadata")
    table.add_column("key")
    table.add_column("value", overflow="fold")

    for kv in kvs:
        table.add_row(kv.key, kv.value)

    RICH_CONSOLE.print(table)

def _print_schema(schema: List[ttypes.SchemaElement]):
    table = Table(title="Schema Elements")
    table.add_column("name")
    table.add_column("type")
    table.add_column("type_length")
    table.add_column("num_children")
    table.add_column("repetition_type")
    table.add_column("converted_type")

    for schema_element in schema:
        table.add_row(
            schema_element.name,
            _resolve_enum(schema_element.type, ttypes.Type),
            str(schema_element.type_length),
            str(schema_element.num_children),
            _resolve_enum(schema_element.repetition_type, ttypes.FieldRepetitionType),
            _resolve_enum(schema_element.converted_type, ttypes.ConvertedType),
        )

    RICH_CONSOLE.print(table)

def _print_row_group(i: int, row_group: ttypes.RowGroup):
    table = Table(title=f"Row Group {i}", show_header=False)
    table.add_row("num_rows", str(row_group.num_rows))
    table.add_row("file_offset", str(row_group.file_offset))
    table.add_row("total_byte_size", str(row_group.total_byte_size))
    table.add_row("total_compressed_size", str(row_group.total_compressed_size))
    table.add_row("ordinal", str(row_group.ordinal))

    RICH_CONSOLE.print(table)

    for column in row_group.columns:
        _print_column_metadata(column)

def _print_column_metadata(column: ttypes.ColumnChunk):
    metadata = column.meta_data

    table = Table(title="Column - {}".format(metadata.path_in_schema), show_header=False)
    table.add_column("key")
    table.add_column("value", overflow="fold")

    table.add_row("file_offset", str(column.file_offset))
    table.add_row("offset_index_offset", str(column.offset_index_offset))
    table.add_row("offset_index_length", str(column.offset_index_length))
    table.add_row("column_index_offset", str(column.column_index_offset))
    table.add_row("column_index_length", str(column.column_index_length))

    table.add_row("type", _resolve_enum(metadata.type, ttypes.Type))
    table.add_row("encodings", str(metadata.encodings))
    table.add_row("path_in_schema", str(metadata.path_in_schema))
    table.add_row("codec", _resolve_enum(metadata.codec, ttypes.CompressionCodec))
    table.add_row("num_values", str(metadata.num_values))
    table.add_row("total_uncompressed_size", str(metadata.total_uncompressed_size))
    table.add_row("total_compressed_size", str(metadata.total_compressed_size))
    table.add_row("key_value_metadata", str(metadata.key_value_metadata))
    table.add_row("data_page_offset", str(metadata.data_page_offset))
    table.add_row("index_page_offset", str(metadata.index_page_offset))
    table.add_row("dictionary_page_offset", str(metadata.dictionary_page_offset))
    table.add_row("statistics", _stats_table(metadata.statistics))
    table.add_row("encoding_stats", str(metadata.encoding_stats))
    table.add_row("bloom_filter_offset", str(metadata.bloom_filter_offset))
    table.add_row("bloom_filter_length", str(metadata.bloom_filter_length))
    table.add_row("size_statistics", str(metadata.size_statistics))

    RICH_CONSOLE.print(table)

def _stats_table(stats: ttypes.Statistics):
    table = Table(show_header=False)
    table.add_column("key")
    table.add_column("value", overflow="fold")

    keys = [
        "max",
        "min",
        "null_count",
        "distinct_count",
        "max_value",
        "min_value",
        "is_max_value_exact",
        "is_min_value_exact",
    ]

    for key in keys:
        value = getattr(stats, key)
        table.add_row(key, str(value))

    return table


def print_metadata(m: ttypes.FileMetaData):
    _print_info(m)
    _print_schema(m.schema)
    _print_key_value_metadata(m.key_value_metadata)

    for (i, row_group) in enumerate(m.row_groups):
        _print_row_group(i, row_group)

    print("\n")
    print(f"column_orders: {m.column_orders}")
    print(f"encryption_algorithm: {m.encryption_algorithm}")
    print(f"footer_signing_key_metadata: {m.footer_signing_key_metadata}")
