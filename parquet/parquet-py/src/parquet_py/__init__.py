from .parquet import ttypes
from .metadata import print_metadata
from . import exceptions

from thrift.protocol import TCompactProtocol
from thrift.transport import TTransport
import os
from pathlib import Path

# Parquet files start and end with this magic bytes.
MAGIC = b"PAR1"

# Footer metadata length 4 bytes and stored before footer MAGIC.
METADATA_LENGTH = 4
METADATA_LENGTH_START = len(MAGIC) + METADATA_LENGTH


def _check_magic(file, offset, whence) -> bool:
    file.seek(offset, whence)
    magic = file.read(len(MAGIC))
    return magic == MAGIC


def _check_magic_header(file) -> bool:
    return _check_magic(file, 0, 0)


def _check_magic_footer(file) -> bool:
    return _check_magic(file, -len(MAGIC), 2)


def _read_thrift_structure(file, struct_class) -> ttypes.FileMetaData:
    transport = TTransport.TFileObjectTransport(file)
    # All thrift structures are serialized using TCompactProtocol.
    protocol = TCompactProtocol.TCompactProtocol(transport)
    structure = struct_class()
    structure.read(protocol)

    return structure


def _read_metadata(file) -> ttypes.FileMetaData:
    # Seek to read metadata length.
    file.seek(-METADATA_LENGTH_START, 2)
    # Metadata is stored as little endian.
    metadata_length = int.from_bytes(file.read(METADATA_LENGTH), "little")

    # Seek to read metadata
    file.seek(-(METADATA_LENGTH_START + metadata_length), 2)
    # Metadata is stored as a thrift structure.
    metadata = _read_thrift_structure(file, ttypes.FileMetaData)

    return metadata


def _project_dir() -> Path:
    return Path(__file__).parent.parent.parent


def _test_data(filename):
    return os.path.join(_project_dir(), "test-data", filename)

def _read_column_index(file, cc: ttypes.ColumnChunk) -> ttypes.ColumnIndex:
    offset = cc.column_index_offset
    file.seek(offset)
    index = _read_thrift_structure(file, ttypes.ColumnIndex)
    return index

def _read_offset_index(file, cc: ttypes.ColumnChunk) -> ttypes.OffsetIndex:
    offset = cc.offset_index_offset
    file.seek(offset)
    index = _read_thrift_structure(file, ttypes.OffsetIndex)
    return index

def print_page_header(file, offset):
    file.seek(offset)
    header = _read_thrift_structure(file, ttypes.PageHeader)
    print(header)
    print()


def read_example():
    sample_parquet = _test_data("nation.parquet")

    with open(sample_parquet, "rb") as f:
        if not _check_magic_header(f):
            raise exceptions.InvalidParquetFile("Missing header magic")
        if not _check_magic_footer(f):
            raise exceptions.InvalidParquetFile("Missing footer magic")
        metadata = _read_metadata(f)
        print_metadata(metadata)

        print("\n--- index ---\n")
        for row_group in metadata.row_groups:
            for cc in row_group.columns:
                column_index = _read_column_index(f, cc)
                offset_index = _read_offset_index(f, cc)

                print("column index: {}\n".format(column_index))
                print("offset index: {}\n".format(offset_index))


        print("\n--- page-headers ---\n")
        for row_group in metadata.row_groups:
            for cc in row_group.columns:
                data_page_offset = cc.meta_data.data_page_offset
                dictionary_page_offset = cc.meta_data.dictionary_page_offset

                print_page_header(f, data_page_offset)
                print_page_header(f, dictionary_page_offset)
