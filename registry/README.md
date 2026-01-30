# mdlibs-Registry Server

This is the self-hosted registry server component of mdlibs. It provides a centralized service for publishing, discovering, and managing markdown libraries within organizations.

## Quick Start

### Using Docker Compose (Recommended)

1. Copy the example environment file:
```bash
cp ../.env.example ../.env
```

2. Edit `.env` and set secure passwords:
```bash
# Generate a secure JWT secret
JWT_SECRET=$(openssl rand -base64 32)

# Set strong passwords
POSTGRES_PASSWORD=your-secure-password
ADMIN_PASSWORD=your-admin-password
```

3. Start the registry:
```bash
cd ..
docker-compose up -d
```

4. Verify it's running:
```bash
curl http://localhost:8080/health
```

See [DEPLOYMENT.md](../docs/registry/DEPLOYMENT.md) for detailed deployment instructions.

## Features

- **Library Management**: Publish, search, download, and manage markdown libraries
- **Authentication**: Secure API token-based authentication
- **User Scoping**: Each user has their own namespace for libraries
- **Team Support**: Create teams and share libraries within teams (coming in v1.1)
- **Version Control**: Semantic versioning support for libraries
- **Search & Discovery**: Full-text search across library names, descriptions, and tags
- **Usage Analytics**: Track downloads and usage patterns
- **Docker Deployment**: Easy deployment with Docker and Docker Compose
- **Multiple Databases**: Support for PostgreSQL and SQLite

## Development

### Prerequisites

- Rust 1.74+
- PostgreSQL 15+ or SQLite 3+
- Docker (optional, for containerized development)

### Build from Source

```bash
cd registry
cargo build --release
```

### Run Locally

1. Set up database:
```bash
# PostgreSQL
createdb mdlibs

# Or use SQLite (no setup needed)
```

2. Set environment variables:
```bash
export DATABASE_URL=postgres://localhost/mdlibs
# Or: export DATABASE_URL=sqlite:///data/mdlibs.db

export JWT_SECRET=$(openssl rand -base64 32)
export STORAGE_PATH=./storage
```

3. Run migrations:
```bash
sqlx migrate run
```

4. Start the server:
```bash
cargo run --release
```

### Run Tests

```bash
cargo test
```

### Development with Docker

```bash
# Build Docker image
docker build -t mdlibs-registry:dev .

# Run with docker-compose
docker-compose -f docker-compose.dev.yml up
```

## API Documentation

The registry provides a RESTful API. See [API.md](../docs/registry/API.md) for complete API documentation.

### Quick API Examples

**Register a user:**
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"user","email":"user@example.com","password":"pass123"}'
```

**Login:**
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"pass123"}'
```

**Publish a library:**
```bash
curl -X POST http://localhost:8080/api/v1/libraries \
  -H "Authorization: Bearer $TOKEN" \
  -F "name=my-lib" \
  -F "version=1.0.0" \
  -F "description=My library" \
  -F "file=@library.tar.gz"
```

**Search libraries:**
```bash
curl http://localhost:8080/api/v1/libraries?q=documentation
```

## Configuration

The registry can be configured via environment variables or a `config.toml` file.

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | - | Database connection string (required) |
| `JWT_SECRET` | - | Secret for JWT tokens (required) |
| `REGISTRY_HOST` | `0.0.0.0` | Host to bind to |
| `REGISTRY_PORT` | `8080` | Port to listen on |
| `STORAGE_PATH` | `/var/lib/mdlibs/storage` | Path for library storage |
| `STORAGE_MAX_FILE_SIZE_MB` | `100` | Max library size in MB |
| `LOG_LEVEL` | `info` | Log level (error, warn, info, debug) |

See [DEPLOYMENT.md](../docs/registry/DEPLOYMENT.md) for full configuration reference.

## Architecture

```
Registry Server
├── API Layer (actix-web)
│   ├── Authentication endpoints
│   ├── Library management endpoints
│   ├── Team management endpoints
│   └── Statistics endpoints
├── Business Logic
│   ├── User management
│   ├── Token management
│   ├── Library validation
│   └── Access control
├── Data Layer
│   ├── Database (PostgreSQL/SQLite)
│   └── File storage
└── Infrastructure
    ├── Logging (tracing)
    ├── Error handling
    └── Configuration
```

See [ARCHITECTURE.md](../docs/registry/ARCHITECTURE.md) for detailed architecture documentation.

## Project Structure

```
registry/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration management
│   ├── api/                 # API handlers
│   │   ├── mod.rs
│   │   ├── auth.rs          # Authentication endpoints
│   │   ├── libraries.rs     # Library endpoints
│   │   ├── teams.rs         # Team endpoints
│   │   └── health.rs        # Health check
│   ├── db/                  # Database layer
│   │   ├── mod.rs
│   │   ├── models.rs        # Database models
│   │   └── schema.rs        # Schema definitions
│   ├── storage/             # Storage layer
│   │   ├── mod.rs
│   │   └── filesystem.rs    # Filesystem storage
│   └── auth/                # Authentication
│       ├── mod.rs
│       └── tokens.rs        # Token management
├── migrations/              # Database migrations
├── tests/                   # Integration tests
├── Dockerfile
├── Cargo.toml
└── README.md
```

## Documentation

- [Requirements](../docs/registry/REQUIREMENTS.md) - Functional and non-functional requirements
- [Architecture](../docs/registry/ARCHITECTURE.md) - System architecture and design
- [API Documentation](../docs/registry/API.md) - Complete API reference
- [Deployment Guide](../docs/registry/DEPLOYMENT.md) - Deployment instructions
- [CLI Integration](../docs/registry/CLI_INTEGRATION.md) - Using the CLI with the registry
- [MVP Plan](../docs/registry/MVP_PLAN.md) - Development roadmap

## Security

- Passwords are hashed with Argon2
- API tokens are hashed before storage
- All sensitive operations require authentication
- Rate limiting on all endpoints
- Input validation and sanitization
- SQL injection prevention (parameterized queries)

For security issues, please see [SECURITY.md](../SECURITY.md).

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Support

- Issues: https://github.com/eisenhowerj/mdlibs/issues
- Documentation: https://github.com/eisenhowerj/mdlibs/tree/main/docs/registry
- Discussions: https://github.com/eisenhowerj/mdlibs/discussions

## Roadmap

### v1.0 (MVP) - Current
- [x] User authentication
- [x] Library publishing
- [x] Library search and download
- [x] Basic usage tracking
- [x] Docker deployment
- [ ] Complete documentation

### v1.1 - Planned
- [ ] Team support
- [ ] Team-scoped libraries
- [ ] Enhanced search
- [ ] Usage analytics dashboard

### v2.0 - Future
- [ ] OAuth2/OIDC integration
- [ ] S3-compatible storage
- [ ] Web UI
- [ ] Library dependency management

## Acknowledgments

Built with:
- [actix-web](https://actix.rs/) - Fast web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [tokio](https://tokio.rs/) - Async runtime
- And many other great Rust crates!
