# mdlibs

A markdown library and document management CLI tool written in Rust.

## Overview

`mdlibs` is a command-line tool for managing collections of markdown documents. It provides functionality to initialize libraries, list documents, update metadata, and search through your markdown files.

## Installation

### From Source

```bash
git clone https://github.com/eisenhowerj/mdlibs.git
cd mdlibs
cargo build --release
```

The binary will be available at `target/release/mdlibs`.

## Usage

### Initialize a new markdown library

```bash
mdlibs init [path]
```

Initialize a new markdown library at the specified path (defaults to current directory).

### List documents

```bash
mdlibs list [--filter <filter>]
```

List all markdown documents in the library. Optionally filter by tag or category.

### Update documents

```bash
mdlibs update <document> [--title <title>]
```

Update metadata or content of a markdown document.

### Search documents

```bash
mdlibs search <query> [--title-only]
```

Search through markdown documents. Use `--title-only` to search only in document titles.

## Examples

```bash
# Initialize a library in the current directory
mdlibs init

# Initialize a library in a specific directory
mdlibs init ~/my-notes

# List all documents
mdlibs list

# List documents with a filter
mdlibs list --filter "tutorial"

# Update a document's title
mdlibs update doc1.md --title "New Title"

# Search for content
mdlibs search "rust programming"

# Search only in titles
mdlibs search "tutorial" --title-only
```

## Development

### Building

```bash
cargo build
```

### Running tests

```bash
cargo test
```

### Running the CLI

```bash
cargo run -- <command> [args]
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.
