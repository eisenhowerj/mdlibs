# mdlibs-Registry Deployment Guide

## Overview

This guide covers deploying the mdlibs-Registry in various environments. The registry is designed to be easy to deploy using Docker and supports both PostgreSQL and SQLite databases.

## Prerequisites

- Docker 20.0+ and Docker Compose 2.0+
- At least 2GB RAM and 10GB disk space
- Open port 8080 (or your configured port)
- (Optional) PostgreSQL 15+ if not using Docker Compose
- (Optional) Reverse proxy (nginx, traefik) for HTTPS

## Quick Start with Docker Compose

### 1. Download Configuration Files

Create a directory for your registry:

```bash
mkdir mdlibs-registry
cd mdlibs-registry
```

Download the sample `docker-compose.yml` and configuration files from the repository.

### 2. Configure Environment Variables

Create a `.env` file:

```bash
# Registry Configuration
REGISTRY_HOST=0.0.0.0
REGISTRY_PORT=8080
REGISTRY_URL=https://registry.example.com

# Database Configuration (PostgreSQL)
DATABASE_URL=postgres://mdlibs:secret@postgres:5432/mdlibs
# Or for SQLite:
# DATABASE_URL=sqlite:///data/mdlibs.db

# Storage Configuration
STORAGE_PATH=/var/lib/mdlibs/storage
STORAGE_MAX_FILE_SIZE_MB=100

# Security Configuration
JWT_SECRET=<generate-a-secure-random-string>
TOKEN_EXPIRY_DAYS=90

# Rate Limiting
RATE_LIMIT_ANON=60
RATE_LIMIT_AUTH=5000

# Admin User (created on first start)
ADMIN_USERNAME=admin
ADMIN_EMAIL=admin@example.com
ADMIN_PASSWORD=<change-me>

# PostgreSQL Configuration
POSTGRES_USER=mdlibs
POSTGRES_PASSWORD=secret
POSTGRES_DB=mdlibs
```

**Important**: Change the default passwords and generate a secure JWT secret:

```bash
# Generate JWT secret
openssl rand -base64 32
```

### 3. Start the Registry

```bash
docker-compose up -d
```

This will:
- Start PostgreSQL database
- Run database migrations
- Start the registry server
- Create the admin user

### 4. Verify Installation

Check that the registry is running:

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "checks": {
    "database": "ok",
    "storage": "ok"
  }
}
```

### 5. Create Your First User

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "yourname",
    "email": "you@example.com",
    "password": "SecurePassword123!"
  }'
```

### 6. Get an API Token

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "yourname",
    "password": "SecurePassword123!"
  }'
```

Save the returned token for CLI configuration.

## Deployment Options

### Option 1: Docker Compose (Recommended)

**Best for**: Production deployments, teams, multi-user setups

**Pros**:
- Easy to set up and maintain
- Includes PostgreSQL database
- Persistent storage via volumes
- Easy to upgrade

**Cons**:
- Requires Docker and Docker Compose

See "Quick Start" section above for instructions.

### Option 2: Standalone Docker Container

**Best for**: Integration with existing database, custom setups

```bash
docker run -d \
  --name mdlibs-registry \
  -p 8080:8080 \
  -v /path/to/storage:/var/lib/mdlibs/storage \
  -e DATABASE_URL=postgres://user:pass@host:5432/mdlibs \
  -e JWT_SECRET=your-secret \
  -e STORAGE_PATH=/var/lib/mdlibs/storage \
  mdlibs/registry:latest
```

### Option 3: SQLite (Small Deployments)

For small teams or personal use, SQLite is simpler:

```bash
docker run -d \
  --name mdlibs-registry \
  -p 8080:8080 \
  -v /path/to/data:/data \
  -e DATABASE_URL=sqlite:///data/mdlibs.db \
  -e JWT_SECRET=your-secret \
  -e STORAGE_PATH=/data/storage \
  mdlibs/registry:latest
```

### Option 4: Native Binary (Advanced)

**Best for**: Custom deployments, development

1. Build from source:
```bash
cd registry
cargo build --release
```

2. Run migrations:
```bash
sqlx migrate run
```

3. Start the server:
```bash
./target/release/mdlibs-registry
```

## Configuration Reference

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `REGISTRY_HOST` | `0.0.0.0` | Host to bind to |
| `REGISTRY_PORT` | `8080` | Port to listen on |
| `REGISTRY_URL` | - | Public URL of registry |
| `DATABASE_URL` | - | Database connection string (required) |
| `STORAGE_PATH` | `/var/lib/mdlibs/storage` | Path for library storage |
| `STORAGE_MAX_FILE_SIZE_MB` | `100` | Max library file size (MB) |
| `JWT_SECRET` | - | Secret for JWT tokens (required) |
| `TOKEN_EXPIRY_DAYS` | `90` | Default token expiry (days) |
| `RATE_LIMIT_ANON` | `60` | Rate limit for anonymous (per hour) |
| `RATE_LIMIT_AUTH` | `5000` | Rate limit for authenticated (per hour) |
| `LOG_LEVEL` | `info` | Logging level (error, warn, info, debug) |
| `LOG_FORMAT` | `json` | Log format (json, pretty) |
| `CORS_ALLOWED_ORIGINS` | `*` | Allowed CORS origins |
| `ADMIN_USERNAME` | - | Initial admin username |
| `ADMIN_EMAIL` | - | Initial admin email |
| `ADMIN_PASSWORD` | - | Initial admin password |

### Configuration File

Alternatively, create `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080
url = "https://registry.example.com"

[database]
url = "postgres://mdlibs:secret@localhost:5432/mdlibs"
max_connections = 10

[storage]
path = "/var/lib/mdlibs/storage"
max_file_size_mb = 100

[security]
jwt_secret = "your-secret-here"
token_expiry_days = 90

[rate_limit]
anonymous = 60
authenticated = 5000

[logging]
level = "info"
format = "json"
```

## Production Deployment

### HTTPS with Nginx

Create an nginx configuration:

```nginx
server {
    listen 443 ssl http2;
    server_name registry.example.com;
    
    ssl_certificate /etc/ssl/certs/registry.crt;
    ssl_certificate_key /etc/ssl/private/registry.key;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Increase for large file uploads
        client_max_body_size 100M;
    }
}
```

### HTTPS with Traefik

Add labels to docker-compose.yml:

```yaml
services:
  registry:
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.registry.rule=Host(`registry.example.com`)"
      - "traefik.http.routers.registry.tls=true"
      - "traefik.http.routers.registry.tls.certresolver=letsencrypt"
```

### Database Backups

#### PostgreSQL Backups

**Automated backups**:
```bash
# Add to crontab
0 2 * * * docker exec mdlibs-postgres pg_dump -U mdlibs mdlibs > /backups/mdlibs-$(date +\%Y\%m\%d).sql
```

**Manual backup**:
```bash
docker exec mdlibs-postgres pg_dump -U mdlibs mdlibs > backup.sql
```

**Restore from backup**:
```bash
docker exec -i mdlibs-postgres psql -U mdlibs mdlibs < backup.sql
```

#### SQLite Backups

```bash
# Backup
docker exec mdlibs-registry sqlite3 /data/mdlibs.db ".backup '/data/backup.db'"

# Or copy the file
docker cp mdlibs-registry:/data/mdlibs.db ./backup.db
```

### Storage Backups

```bash
# Backup storage directory
tar -czf storage-backup-$(date +%Y%m%d).tar.gz /path/to/storage

# Or use rsync
rsync -av /path/to/storage/ /backup/location/
```

### Monitoring

#### Health Checks

Add to monitoring system:
```bash
curl https://registry.example.com/health
```

#### Prometheus Metrics

Configure Prometheus to scrape metrics:

```yaml
scrape_configs:
  - job_name: 'mdlibs-registry'
    static_configs:
      - targets: ['registry.example.com:8080']
    metrics_path: /metrics
```

### Log Management

#### View Logs

```bash
# Docker Compose
docker-compose logs -f registry

# Standalone Docker
docker logs -f mdlibs-registry
```

#### Log Rotation

Configure in docker-compose.yml:

```yaml
services:
  registry:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

### Resource Requirements

#### Minimum Requirements
- **CPU**: 1 core
- **RAM**: 1GB
- **Disk**: 10GB (+ storage for libraries)
- **Network**: 10 Mbps

#### Recommended for Production
- **CPU**: 2+ cores
- **RAM**: 4GB
- **Disk**: 50GB SSD (+ storage for libraries)
- **Network**: 100 Mbps
- **Database**: Separate PostgreSQL instance

#### Scaling Considerations
- Libraries are stored on disk, ensure sufficient storage
- Database size grows with users and metadata
- Plan for ~1MB database per 100 libraries
- Storage needed = sum of all library sizes

## Upgrades

### Upgrading the Registry

1. **Backup everything**:
```bash
docker-compose down
# Backup database and storage (see above)
```

2. **Pull new image**:
```bash
docker-compose pull
```

3. **Run migrations** (if needed):
```bash
docker-compose run registry sqlx migrate run
```

4. **Start updated version**:
```bash
docker-compose up -d
```

### Downgrading

If you need to rollback:

1. Stop the registry
2. Restore database and storage from backup
3. Use previous Docker image version:
```bash
docker-compose down
docker-compose up -d mdlibs/registry:1.0.0
```

## Troubleshooting

### Registry Won't Start

**Check logs**:
```bash
docker-compose logs registry
```

**Common issues**:
- Database connection failed: Check `DATABASE_URL` and database is running
- Permission denied: Check volume mount permissions
- Port already in use: Change `REGISTRY_PORT` or stop conflicting service

### Database Connection Issues

**PostgreSQL**:
```bash
# Test connection
docker exec mdlibs-postgres psql -U mdlibs -d mdlibs -c "SELECT 1;"

# Check if database exists
docker exec mdlibs-postgres psql -U mdlibs -l
```

**SQLite**:
```bash
# Check database file
docker exec mdlibs-registry ls -la /data/mdlibs.db

# Test database
docker exec mdlibs-registry sqlite3 /data/mdlibs.db "SELECT 1;"
```

### Storage Issues

**Check permissions**:
```bash
docker exec mdlibs-registry ls -la /var/lib/mdlibs/storage
```

**Check disk space**:
```bash
df -h
```

### Performance Issues

**Check resource usage**:
```bash
docker stats mdlibs-registry
```

**Optimize database** (PostgreSQL):
```bash
docker exec mdlibs-postgres psql -U mdlibs -d mdlibs -c "VACUUM ANALYZE;"
```

### Can't Login or Authenticate

**Reset user password**:
```bash
# Will add password reset CLI command in future version
# For now, directly update database (PostgreSQL):
docker exec mdlibs-postgres psql -U mdlibs -d mdlibs
# Then run UPDATE query with new hashed password
```

## Security Best Practices

1. **Change all default passwords** immediately
2. **Use strong JWT_SECRET** (at least 32 random bytes)
3. **Enable HTTPS** in production (use reverse proxy)
4. **Restrict database access** to localhost or private network
5. **Regular backups** of database and storage
6. **Keep Docker images updated** for security patches
7. **Use firewall rules** to restrict access
8. **Monitor logs** for suspicious activity
9. **Implement rate limiting** at reverse proxy level
10. **Regular security audits** of deployed system

## High Availability Setup

For enterprise deployments requiring high availability:

### Load Balanced Setup

```
                   ┌──────────────┐
                   │ Load Balancer│
                   └──────┬───────┘
                          │
            ┌─────────────┼─────────────┐
            │             │             │
       ┌────▼───┐    ┌────▼───┐    ┌───▼────┐
       │Registry│    │Registry│    │Registry│
       │   #1   │    │   #2   │    │   #3   │
       └────┬───┘    └────┬───┘    └───┬────┘
            │             │             │
            └─────────────┼─────────────┘
                          │
                ┌─────────▼──────────┐
                │  PostgreSQL        │
                │  (Primary/Replica) │
                └─────────┬──────────┘
                          │
                ┌─────────▼──────────┐
                │  Shared Storage    │
                │  (NFS/S3)          │
                └────────────────────┘
```

**Requirements**:
- Shared storage (NFS, S3, etc.)
- PostgreSQL with replication
- Load balancer (nginx, haproxy, cloud LB)
- Session affinity not required (stateless)

## Support

For issues and questions:
- GitHub Issues: https://github.com/eisenhowerj/mdlibs/issues
- Documentation: https://github.com/eisenhowerj/mdlibs/tree/main/docs/registry
- Community: [Add your community links]

## Next Steps

- [Configure CLI to use your registry](CLI_INTEGRATION.md)
- [Learn about the API](API.md)
- [Understand the architecture](ARCHITECTURE.md)
