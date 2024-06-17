# Parquet Metadata

There are three kinds of metadata.

1. FileMetaData
2. ColumnMetadata (for column chunk)
3. PageHeader


## File Metadata

File metadata is stored in the footer, along with the length.

```
╔══════════════════════════════════╗ 
║┌──────┐                          ║ 
║│ PAR1 │       ...                ║ 
║└──────┘                          ║ 
║               ...                ║ 
║                                  ║ 
║┌────────────────┐┌──────┐┌──────┐║ 
║│    metadata    ││ mlen ││ PAR1 │║ 
║└───────│────────┘└───│──┘└──────┘║ 
╚════════│═════════════│═══════════╝ 
         │             │             
         │             │             
         ▼             ▼             
   file metadata      metadata-length
   (mlen bytes)         (4 bytes)    
```

