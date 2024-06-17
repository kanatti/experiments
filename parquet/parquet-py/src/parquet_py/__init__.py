from .parquet.ttypes import FileMetaData
from thrift.protocol import TCompactProtocol
from thrift.transport import TTransport

# Parquet files start and end with this magic bytes.
MAGIC = b'PAR1'

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

def _read_thrift_structure(file, struct_class) -> any:
    transport = TTransport.TFileObjectTransport(file)
    # All thrift structures are serialized using TCompactProtocol.
    protocol = TCompactProtocol.TCompactProtocol(transport)
    structure = struct_class()
    structure.read(protocol)

    return structure

def _read_metadata(file):
    # Seek to read metadata length.
    file.seek(-METADATA_LENGTH_START, 2)
    # Metadata is stored as little endian.
    metadata_length = int.from_bytes(file.read(METADATA_LENGTH), 'little')

    # Seek to read metadata
    file.seek(-(METADATA_LENGTH_START + metadata_length), 2)
    # Metadata is stored as a thrift structure.
    metadata = _read_thrift_structure(file, FileMetaData)

    return metadata

def print_metadata(m: FileMetaData):
    print(f"version: {m.version}")
    print(f"num_rows: {m.num_rows}")
    print(f"created_by: {m.created_by}")
    print(f"schema: {m.schema}")
    # m.row_groups
    # m.key_value_metadata
    # m.column_orders
    # m.encryption_algorithm
    # m.footer_signing_key_metadata

def read_example():
    sample_parquet = "/Users/balu/Code/experiments/parquet/parquet-py/test-data/nation.parquet"
    with open(sample_parquet, 'rb') as f:
        if not _check_magic_header(f):
            raise Exception("Invalid Parquet file")
        if not _check_magic_footer(f):
            raise Exception("Invalid Parquet file")
        print("Valid Parquet header and footer")
        print("Reading metadata")
        metadata = _read_metadata(f)
        print_metadata(metadata)
