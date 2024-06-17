import thrift

# Parquet files start and end with this magic bytes.
MAGIC = b'PAR1'

def _check_magic(file, offset, whence):
    file.seek(offset, whence)
    magic = file.read(len(MAGIC))
    return magic == MAGIC

def _check_magic_header(file):
    return _check_magic(file, 0, 0)

def _check_magic_footer(file):
    return _check_magic(file, -len(MAGIC), 2)

def read_example():
    sample_parquet = "/Users/balu/Code/experiments/parquet/parquet-py/test-data/nation.parquet"
    with open(sample_parquet, 'rb') as f:
        if not _check_magic_header(f):
            raise Exception("Invalid Parquet file")
        if not _check_magic_footer(f):
            raise Exception("Invalid Parquet file")
        print("Valid Parquet header and footer")