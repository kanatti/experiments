from .parquet import ttypes
from typing import IO


def read_plain(b: IO, num_values: int, physical_type: ttypes.Type):
    match physical_type:
        case ttypes.Type.BOOLEAN:
            return b
        case ttypes.Type.INT32:
            return _read_plain_int32(b, num_values)
        case ttypes.Type.INT64:
            return b
        case ttypes.Type.INT96:
            return b
        case ttypes.Type.FLOAT:
            return b
        case ttypes.Type.DOUBLE:
            return b
        case ttypes.Type.BYTE_ARRAY:
            return _read_plain_byte_array(b, num_values)
        case _:
            raise ValueError(f"Unsupported physical type: {physical_type}")


def _read_plain_byte_array(b: IO, num_values: int) -> list[bytes]:
    def _read_value(b: IO):
        return b.read(int.from_bytes(b.read(4), "little"))

    return [_read_value(b) for _i in range(num_values)]


def _read_plain_int32(b: IO, num_values: int) -> list[int]:
    return [int.from_bytes(b.read(4), "little") for _i in range(num_values)]
