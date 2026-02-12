# Examples

This directory contains example usage scenarios for `antiword-rust`.

## Creating Test Files

To test the converter, you'll need some .doc files. You can:

1. Use legacy Microsoft Word documents you already have
2. Create test .doc files using Microsoft Word or LibreOffice (save in .doc format, not .docx)
3. Download sample .doc files from the internet

## Example Commands

### Single File Conversion

```bash
# Convert a single file to stdout
./target/release/antiword-rust examples/sample.doc

# Convert a single file to a text file
./target/release/antiword-rust examples/sample.doc -o examples/output/
```

### Batch Conversion

```bash
# Convert all .doc files in a directory
./target/release/antiword-rust examples/docs/ -o examples/output/

# Convert all .doc files recursively
./target/release/antiword-rust examples/docs/ -o examples/output/ --recursive

# With verbose output
./target/release/antiword-rust examples/docs/ -o examples/output/ --verbose
```

## Performance Testing

To test parallel processing performance, create a directory with multiple .doc files:

```bash
# Process 100 files in parallel
./target/release/antiword-rust examples/large_batch/ -o examples/output/ --verbose
```

The verbose output will show:
- Total number of files processed
- Processing time
- Success/failure count
- Details of any errors

## Expected Output

When processing files, you'll see output like:

```
Found 50 .doc file(s) to process

Processed 50 file(s) in 2.34s (48 successful, 2 failed)
  ERROR: examples/corrupted.doc - Failed to parse OLE file
  ERROR: examples/invalid.doc - Failed to find WordDocument stream
  OK: examples/doc1.doc -> examples/output/doc1.txt
  OK: examples/doc2.doc -> examples/output/doc2.txt
  ...
```
