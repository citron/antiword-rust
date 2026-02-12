# antiword-rust

A fast, parallelized Microsoft Word (.doc) to text converter written in Rust.

## Overview

`antiword-rust` is a modern Rust implementation inspired by the classic [antiword](https://github.com/grobian/antiword) tool. It converts legacy Microsoft Word documents (.doc format) to plain text with a focus on speed and parallel processing.

## Features

- **🚀 Fast**: Built with Rust for maximum performance
- **⚡ Parallel Processing**: Utilizes multiple CPU cores via [rayon](https://github.com/rayon-rs/rayon) to convert multiple files simultaneously
- **📁 Batch Conversion**: Process entire directories of .doc files at once
- **🔄 Recursive Search**: Automatically find and process .doc files in subdirectories
- **💪 Robust**: Comprehensive error handling with detailed error messages
- **🧪 Tested**: Includes unit tests for core functionality

## Installation

### From Source

```bash
git clone https://github.com/citron/antiword-rust.git
cd antiword-rust
cargo build --release
```

The compiled binary will be available at `target/release/antiword-rust`.

## Usage

### Convert a Single File to stdout

```bash
antiword-rust document.doc
```

### Convert a Single File to a Text File

```bash
antiword-rust document.doc -o output_dir/
```

This will create `output_dir/document.txt`.

### Convert Multiple Files in a Directory

```bash
antiword-rust /path/to/docs/ -o /path/to/output/
```

### Recursive Directory Processing

```bash
antiword-rust /path/to/docs/ -o /path/to/output/ --recursive
```

### Verbose Output

```bash
antiword-rust /path/to/docs/ -o /path/to/output/ --verbose
```

## Command-Line Options

- `<INPUT>`: Input file or directory containing .doc files (required)
- `-o, --output <DIR>`: Output directory for converted text files (optional, prints to stdout if not specified)
- `-r, --recursive`: Process files recursively when input is a directory
- `-v, --verbose`: Show verbose output including processing time and file-by-file status
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Performance

`antiword-rust` leverages Rust's performance characteristics and parallel processing capabilities:

- **Parallel Processing**: When converting multiple files, the tool automatically distributes the work across available CPU cores
- **Memory Efficient**: Processes files in a streaming fashion where possible
- **Optimized Binary**: Release builds are compiled with optimizations enabled

### Benchmarks

For processing directories with multiple .doc files, you can expect:
- Linear scaling with the number of CPU cores
- Significantly faster batch processing compared to sequential conversion tools
- Low memory overhead per file

## Architecture

The project is organized into three main modules:

1. **doc_parser**: Core .doc file parsing using the [cfb](https://crates.io/crates/cfb) crate to read OLE compound files
2. **parallel_processor**: Batch processing and parallelization using [rayon](https://crates.io/crates/rayon)
3. **main**: CLI interface built with [clap](https://crates.io/crates/clap)

## Limitations

- Supports legacy .doc format (OLE compound files), not modern .docx files
- Text extraction is simplified and may not preserve all formatting
- Complex documents with embedded objects may not extract perfectly

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Code Style

The project follows standard Rust conventions and uses `cargo fmt` for formatting.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is dual-licensed under MIT OR Apache-2.0.

## Acknowledgments

- Inspired by the original [antiword](https://github.com/grobian/antiword) by Adri van Os
- Built with the excellent Rust ecosystem libraries