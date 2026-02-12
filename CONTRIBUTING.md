# Contributing to antiword-rust

Thank you for your interest in contributing to antiword-rust! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/antiword-rust.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Format code: `cargo fmt`
7. Check for issues: `cargo clippy`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/your-feature-name`
10. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later (install via [rustup](https://rustup.rs/))
- Git

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Examples

```bash
cargo run -- examples/sample.doc
```

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` to format code
- Run `cargo clippy` to catch common mistakes
- Write tests for new functionality
- Document public APIs with doc comments

## Testing Guidelines

- Add unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass before submitting a PR
- Aim for high code coverage

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Write clear, descriptive commit messages
- Update documentation as needed
- Add tests for new functionality
- Ensure all tests pass
- Update CHANGELOG.md if applicable

## Areas for Contribution

We welcome contributions in the following areas:

- **Improved .doc parsing**: Better text extraction algorithms
- **Performance optimizations**: Faster parsing and processing
- **Error handling**: More robust error messages
- **Documentation**: Better examples and guides
- **Tests**: More comprehensive test coverage
- **Features**: Additional output formats, filtering options, etc.

## Reporting Bugs

When reporting bugs, please include:

- Your operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Sample .doc file (if possible)

## Questions?

If you have questions, feel free to:

- Open an issue for discussion
- Check existing issues and PRs
- Read the documentation in README.md

## License

By contributing to antiword-rust, you agree that your contributions will be licensed under both the MIT License and Apache License 2.0.
