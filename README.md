# mdlibs

A markdown library and document management CLI tool with self-hosted registry support, written in Rust.

## Overview

`mdlibs` is a command-line tool for managing collections of markdown documents. It provides functionality to initialize libraries, list documents, update metadata, and search through your markdown files.

**New in Phase 3**: Self-hosted registry support for publishing and sharing markdown libraries within organizations.

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

## Self-Hosted Registry (Phase 3)

mdlibs now includes a self-hosted registry server for enterprise and corporate usage. The registry enables teams to:

- **Publish** markdown libraries to a centralized repository
- **Search** and discover libraries across your organization
- **Install** libraries directly from the registry
- **Collaborate** with team-based library scoping
- **Track** usage and download analytics

### Quick Start with Registry

1. **Deploy the registry** using Docker Compose:
```bash
cp .env.example .env
# Edit .env with your configuration
docker-compose up -d
```

2. **Configure the CLI** to use your registry:
```bash
mdlibs login https://registry.example.com
```

3. **Publish a library**:
```bash
mdlibs publish
```

4. **Search and install**:
```bash
mdlibs search documentation
mdlibs install @user/my-docs
```

### Registry Documentation

- **[Requirements](docs/registry/REQUIREMENTS.md)** - Functional and non-functional requirements
- **[Architecture](docs/registry/ARCHITECTURE.md)** - System architecture and design decisions
- **[API Specification](docs/registry/API.md)** - Complete REST API documentation
- **[Deployment Guide](docs/registry/DEPLOYMENT.md)** - How to deploy the registry
- **[CLI Integration](docs/registry/CLI_INTEGRATION.md)** - Using the CLI with the registry
- **[MVP Plan](docs/registry/MVP_PLAN.md)** - Development roadmap and timeline

### Registry Features

#### MVP (v1.0) - In Progress
- ✅ Requirements and architecture design
- ✅ API specification
- ✅ Deployment infrastructure (Docker)
- 🔄 User authentication and API tokens
- 🔄 Library publishing and downloading
- 🔄 Search and discovery
- 🔄 CLI integration

#### Planned (v1.1+)
- Team support and collaboration
- Advanced analytics and reporting
- OAuth2/OIDC integration
- S3-compatible storage backend
- Web UI for registry management

## Development

### Building the CLI

```bash
cargo build
```

### Building the Registry

```bash
cd registry
cargo build --release
```

### Running tests

```bash
# Test CLI
cargo test

# Test Registry
cd registry
cargo test
```

### Running the CLI

```bash
cargo run -- <command> [args]
```

### Running the Registry Locally

```bash
cd registry
export DATABASE_URL=sqlite:///tmp/mdlibs.db
export JWT_SECRET=$(openssl rand -base64 32)
cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.
