class ParquetException(Exception):
    """Base Parquet Exception"""
    message: str

    def __init__(self, message: str):
        self.message = message or self.message
        super().__init__(self.message)

class InvalidParquetFile(ParquetException):
    """Raised when a parquet file is invalid"""
    message = "Invalid Parquet file"