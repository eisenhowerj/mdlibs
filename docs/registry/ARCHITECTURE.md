# mdlibs-Registry Architecture

## System Overview

The mdlibs-Registry is a self-hosted server application that provides centralized storage and discovery for markdown libraries. It consists of:

1. **Registry Server**: RESTful API service (Rust/Actix-web)
2. **Database**: PostgreSQL or SQLite for metadata
3. **Storage Backend**: File system or S3 for library archives
4. **CLI Client**: Extended mdlibs CLI with registry support

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         mdlibs CLI                               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │  publish │  │  install │  │  search  │  │   login  │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
└───────┼─────────────┼─────────────┼─────────────┼──────────────┘
        │             │             │             │
        │             │             │             │ HTTPS/REST API
        └─────────────┴─────────────┴─────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                    mdlibs-Registry Server                        │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                     API Layer (Actix-web)                 │  │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐         │  │
│  │  │ Auth   │  │ Publish│  │ Search │  │ Admin  │         │  │
│  │  └───┬────┘  └───┬────┘  └───┬────┘  └───┬────┘         │  │
│  └──────┼───────────┼───────────┼───────────┼──────────────┘  │
│         │           │           │           │                  │
│  ┌──────┼───────────┼───────────┼───────────┼──────────────┐  │
│  │      │           │           │           │               │  │
│  │  ┌───▼───────────▼───────────▼───────────▼────┐         │  │
│  │  │          Business Logic Layer              │         │  │
│  │  │  • Authentication & Authorization          │         │  │
│  │  │  • Library Management                      │         │  │
│  │  │  • Team Management                         │         │  │
│  │  │  • Usage Tracking                          │         │  │
│  │  └────────────┬───────────────┬───────────────┘         │  │
│  └───────────────┼───────────────┼─────────────────────────┘  │
│                  │               │                             │
│  ┌───────────────▼──────┐  ┌─────▼──────────────┐            │
│  │   Database Layer     │  │   Storage Layer    │            │
│  │  • Users/Tokens      │  │  • Library Files   │            │
│  │  • Libraries         │  │  • Tar Archives    │            │
│  │  • Teams             │  │                    │            │
│  │  • Analytics         │  │                    │            │
│  └──────────┬───────────┘  └─────┬──────────────┘            │
└─────────────┼──────────────────────┼──────────────────────────┘
              │                      │
              ▼                      ▼
     ┌────────────────┐    ┌──────────────────┐
     │   PostgreSQL   │    │  File System or  │
     │   or SQLite    │    │   S3 Storage     │
     └────────────────┘    └──────────────────┘
```

## Component Details

### 1. Registry Server

#### 1.1 Technology Stack
- **Language**: Rust (for consistency with CLI)
- **Web Framework**: Actix-web (high-performance async HTTP)
- **Database ORM**: Diesel or SQLx
- **Authentication**: JWT tokens or API keys
- **Serialization**: Serde (JSON)

#### 1.2 API Endpoints

##### Authentication & Users
```
POST   /api/v1/auth/register          - Register new user
POST   /api/v1/auth/login             - Login and get token
POST   /api/v1/auth/logout            - Revoke token
GET    /api/v1/auth/tokens            - List user tokens
POST   /api/v1/auth/tokens            - Create new token
DELETE /api/v1/auth/tokens/:id        - Revoke token
GET    /api/v1/users/me               - Get current user info
```

##### Library Management
```
GET    /api/v1/libraries              - Search/list libraries
GET    /api/v1/libraries/:scope/:name - Get library details
POST   /api/v1/libraries              - Publish new library
GET    /api/v1/libraries/:scope/:name/:version - Get specific version
DELETE /api/v1/libraries/:scope/:name/:version - Delete version
GET    /api/v1/libraries/:scope/:name/versions - List all versions
```

##### Library Download
```
GET    /api/v1/libraries/:scope/:name/:version/download - Download library archive
```

##### Team Management
```
GET    /api/v1/teams                  - List user's teams
POST   /api/v1/teams                  - Create new team
GET    /api/v1/teams/:id              - Get team details
PUT    /api/v1/teams/:id              - Update team
DELETE /api/v1/teams/:id              - Delete team
GET    /api/v1/teams/:id/members      - List team members
POST   /api/v1/teams/:id/members      - Add team member
DELETE /api/v1/teams/:id/members/:uid - Remove team member
```

##### Analytics & Monitoring
```
GET    /api/v1/stats/downloads        - Download statistics
GET    /api/v1/stats/libraries        - Library statistics
GET    /health                        - Health check
GET    /metrics                       - Prometheus metrics
```

#### 1.3 API Request/Response Examples

**Publish Library**
```json
POST /api/v1/libraries
Authorization: Bearer <token>
Content-Type: multipart/form-data

{
  "name": "my-docs",
  "version": "1.0.0",
  "description": "My documentation library",
  "tags": ["documentation", "api"],
  "license": "MIT",
  "file": <binary tar.gz>
}

Response: 201 Created
{
  "id": "uuid",
  "scope": "user123",
  "name": "my-docs",
  "version": "1.0.0",
  "created_at": "2026-01-30T10:00:00Z",
  "size": 102400,
  "download_url": "/api/v1/libraries/user123/my-docs/1.0.0/download"
}
```

**Search Libraries**
```json
GET /api/v1/libraries?q=documentation&page=1&per_page=20

Response: 200 OK
{
  "libraries": [
    {
      "scope": "user123",
      "name": "my-docs",
      "latest_version": "1.0.0",
      "description": "My documentation library",
      "downloads": 150,
      "updated_at": "2026-01-30T10:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "per_page": 20
}
```

### 2. Database Schema

#### 2.1 Tables

**users**
```sql
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username VARCHAR(255) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
```

**tokens**
```sql
CREATE TABLE tokens (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  token_hash VARCHAR(255) NOT NULL,
  name VARCHAR(255),
  expires_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL,
  last_used_at TIMESTAMP
);
```

**teams**
```sql
CREATE TABLE teams (
  id UUID PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL,
  display_name VARCHAR(255),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);
```

**team_members**
```sql
CREATE TABLE team_members (
  team_id UUID REFERENCES teams(id),
  user_id UUID REFERENCES users(id),
  role VARCHAR(50) NOT NULL, -- admin, member
  joined_at TIMESTAMP NOT NULL,
  PRIMARY KEY (team_id, user_id)
);
```

**libraries**
```sql
CREATE TABLE libraries (
  id UUID PRIMARY KEY,
  scope_type VARCHAR(50) NOT NULL, -- user, team
  scope_id UUID NOT NULL,
  name VARCHAR(255) NOT NULL,
  version VARCHAR(50) NOT NULL,
  description TEXT,
  tags TEXT[], -- PostgreSQL array
  license VARCHAR(50),
  storage_path VARCHAR(1024) NOT NULL,
  size_bytes BIGINT NOT NULL,
  checksum VARCHAR(64),
  created_at TIMESTAMP NOT NULL,
  created_by UUID REFERENCES users(id),
  UNIQUE (scope_type, scope_id, name, version)
);
```

**downloads**
```sql
CREATE TABLE downloads (
  id UUID PRIMARY KEY,
  library_id UUID REFERENCES libraries(id),
  downloaded_by UUID REFERENCES users(id),
  downloaded_at TIMESTAMP NOT NULL,
  ip_address VARCHAR(45)
);
```

### 3. Storage Backend

#### 3.1 File System Storage
- Libraries stored as `.tar.gz` archives
- Directory structure: `/<scope_type>/<scope_id>/<name>/<version>.tar.gz`
- Example: `/storage/user/abc123/my-docs/1.0.0.tar.gz`

#### 3.2 S3-Compatible Storage (Future)
- Use AWS SDK for Rust
- Bucket structure: `s3://bucket/<scope_type>/<scope_id>/<name>/<version>.tar.gz`
- Support any S3-compatible service (AWS, MinIO, DigitalOcean Spaces)

### 4. Authentication & Authorization

#### 4.1 Authentication Flow
1. User registers or logs in
2. Server generates JWT token or API key
3. Client stores token securely
4. Client includes token in Authorization header for all requests
5. Server validates token and identifies user

#### 4.2 Authorization Rules
- **Anonymous**: Read public libraries only
- **Authenticated**: Read public + own private, publish to own namespace
- **Team Member**: Read team private, publish to team namespace
- **Team Admin**: Manage team members and settings
- **System Admin**: Full access

### 5. Security Considerations

#### 5.1 API Security
- All endpoints use HTTPS/TLS in production
- Rate limiting on all endpoints
- CORS configured appropriately
- Input validation and sanitization
- SQL injection prevention (parameterized queries)

#### 5.2 Authentication Security
- Passwords hashed with Argon2 or bcrypt
- Tokens hashed before storage
- Token expiration enforced
- Audit logging for security events

#### 5.3 Library Security
- File size limits enforced
- Archive validation before storage
- Path traversal prevention
- Malware scanning (optional, future)

### 6. Deployment Architecture

#### 6.1 Single Server Deployment (MVP)
```
┌─────────────────────────────────────┐
│          Docker Host                │
│                                     │
│  ┌──────────────────────────────┐  │
│  │   mdlibs-registry:latest     │  │
│  │   Port: 8080                 │  │
│  └────────┬─────────────────────┘  │
│           │                         │
│  ┌────────▼─────────────────────┐  │
│  │   PostgreSQL:15              │  │
│  │   Port: 5432                 │  │
│  └──────────────────────────────┘  │
│                                     │
│  Volume: /var/lib/mdlibs/storage   │
│  Volume: /var/lib/postgresql/data  │
└─────────────────────────────────────┘
```

#### 6.2 Scalable Deployment (Future)
```
┌──────────────────┐
│   Load Balancer  │
└────────┬─────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼────┐
│ App 1 │ │ App 2 │
└───┬───┘ └──┬────┘
    │        │
    └───┬────┘
        │
   ┌────▼─────┐     ┌──────────┐
   │ Database │     │ S3/MinIO │
   └──────────┘     └──────────┘
```

### 7. Monitoring & Observability

#### 7.1 Metrics
- Request count and latency (per endpoint)
- Library upload/download counts
- Storage usage
- Database connection pool stats
- Error rates

#### 7.2 Logging
- Structured JSON logging
- Log levels: ERROR, WARN, INFO, DEBUG
- Include request ID for tracing
- Separate security audit log

#### 7.3 Health Checks
- `/health` endpoint
- Check database connectivity
- Check storage accessibility
- Return 200 if healthy, 503 if not

## Technology Choices Rationale

### Why Rust?
- Consistency with existing CLI codebase
- High performance and low resource usage
- Strong type safety and memory safety
- Excellent async/await support
- Growing ecosystem for web services

### Why Actix-web?
- One of the fastest web frameworks
- Mature and well-documented
- Good middleware ecosystem
- Active community support

### Why PostgreSQL?
- Robust and reliable
- Excellent JSON support
- Array type for tags
- Full-text search capabilities
- Wide deployment experience

### Why SQLite for Alternative?
- Zero configuration
- Perfect for small deployments
- File-based (easy backup)
- Good for development and testing

## Implementation Roadmap

### Phase 1: MVP (Weeks 1-3)
1. Setup project structure
2. Implement database layer
3. Implement authentication (basic API tokens)
4. Implement library publish/download
5. Implement basic search
6. Create Docker deployment
7. Update CLI with publish/search commands

### Phase 2: Teams & Enhanced Features (Weeks 4-6)
1. Implement team management
2. Add team namespaces
3. Implement usage analytics
4. Add advanced search
5. CLI improvements

### Phase 3: Enterprise Features (Weeks 7-8)
1. Advanced authentication (OAuth2)
2. Quotas and limits
3. Advanced analytics
4. Performance optimization
5. Security hardening

### Phase 4: Production Readiness (Weeks 9-10)
1. Comprehensive testing
2. Load testing
3. Security audit
4. Documentation
5. Deployment automation
