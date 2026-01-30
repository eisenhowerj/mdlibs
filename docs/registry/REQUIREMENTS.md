# mdlibs-Registry Requirements

## Overview

The mdlibs-Registry is a self-hosted service for publishing, discovering, and managing markdown libraries within organizations. It provides a centralized repository for teams to share documentation, templates, and markdown collections.

## Core Requirements

### 1. Library Publishing & Management

#### 1.1 Library Publishing
- **REQ-PUB-001**: Users shall be able to publish markdown libraries to the registry
- **REQ-PUB-002**: Each library shall have a unique identifier (name + owner/team scope)
- **REQ-PUB-003**: Libraries shall support versioning (semantic versioning recommended)
- **REQ-PUB-004**: Publishing shall validate library structure and metadata
- **REQ-PUB-005**: Libraries shall include metadata: name, version, description, author, tags, license

#### 1.2 Library Discovery
- **REQ-DISC-001**: Users shall be able to search libraries by name, description, and tags
- **REQ-DISC-002**: Users shall be able to list all available libraries
- **REQ-DISC-003**: Users shall be able to view library details and metadata
- **REQ-DISC-004**: Search results shall be paginated
- **REQ-DISC-005**: Libraries shall be sortable by name, downloads, last updated

#### 1.3 Library Download
- **REQ-DOWN-001**: Users shall be able to download published libraries
- **REQ-DOWN-002**: Users shall be able to download specific versions
- **REQ-DOWN-003**: Download counts shall be tracked for usage analytics

### 2. Authentication & Authorization

#### 2.1 Authentication
- **REQ-AUTH-001**: Registry shall support API token-based authentication
- **REQ-AUTH-002**: Users shall be able to generate and revoke API tokens
- **REQ-AUTH-003**: Tokens shall have configurable expiration
- **REQ-AUTH-004**: Registry shall support optional integration with external auth (OAuth2/OIDC)

#### 2.2 Authorization
- **REQ-AUTHZ-001**: Anonymous users shall have read-only access (configurable)
- **REQ-AUTHZ-002**: Authenticated users shall be able to publish libraries under their namespace
- **REQ-AUTHZ-003**: Team members shall be able to publish under team namespace
- **REQ-AUTHZ-004**: Library owners shall be able to delete their libraries
- **REQ-AUTHZ-005**: Administrators shall have full access to all operations

### 3. Team & Multi-Tenancy

#### 3.1 Teams
- **REQ-TEAM-001**: Organizations shall be able to create teams
- **REQ-TEAM-002**: Teams shall have unique namespaces for libraries
- **REQ-TEAM-003**: Users shall be members of one or more teams
- **REQ-TEAM-004**: Team libraries can be private or public (configurable)
- **REQ-TEAM-005**: Team admins shall manage team members and permissions

#### 3.2 Scoping
- **REQ-SCOPE-001**: Libraries shall be scoped to user or team namespace
- **REQ-SCOPE-002**: Library names must be unique within a scope
- **REQ-SCOPE-003**: Scoped libraries shall use format: @scope/library-name

### 4. Usage Monitoring & Analytics

#### 4.1 Metrics
- **REQ-MON-001**: Registry shall track download counts per library
- **REQ-MON-002**: Registry shall track API usage per user/team
- **REQ-MON-003**: Registry shall log all publish/delete operations
- **REQ-MON-004**: Administrators shall access usage reports and analytics

#### 4.2 Quotas & Limits
- **REQ-LIMIT-001**: Registry shall support configurable storage quotas per user/team
- **REQ-LIMIT-002**: Registry shall support rate limiting for API endpoints
- **REQ-LIMIT-003**: Registry shall support max library size limits

### 5. Security

#### 5.1 Data Protection
- **REQ-SEC-001**: API tokens shall be stored securely (hashed)
- **REQ-SEC-002**: All API endpoints shall support HTTPS/TLS
- **REQ-SEC-003**: Registry shall validate library contents for security issues
- **REQ-SEC-004**: Registry shall implement audit logging for security events

#### 5.2 Access Control
- **REQ-SEC-005**: Private libraries shall not be accessible without authorization
- **REQ-SEC-006**: Registry shall implement CORS policies
- **REQ-SEC-007**: Registry shall protect against common vulnerabilities (injection, XSS, etc.)

### 6. Deployment & Operations

#### 6.1 Deployment
- **REQ-DEPLOY-001**: Registry shall be deployable via Docker container
- **REQ-DEPLOY-002**: Registry shall support PostgreSQL and SQLite databases
- **REQ-DEPLOY-003**: Registry shall support file system and S3-compatible storage
- **REQ-DEPLOY-004**: Configuration shall be via environment variables and config files

#### 6.2 Operations
- **REQ-OPS-001**: Registry shall provide health check endpoint
- **REQ-OPS-002**: Registry shall provide metrics endpoint (Prometheus-compatible)
- **REQ-OPS-003**: Registry shall support structured logging (JSON)
- **REQ-OPS-004**: Registry shall be horizontally scalable

### 7. CLI Integration

#### 7.1 CLI Commands
- **REQ-CLI-001**: CLI shall support `mdlibs publish` to publish to registry
- **REQ-CLI-002**: CLI shall support `mdlibs install <library>` to download from registry
- **REQ-CLI-003**: CLI shall support `mdlibs search <query>` to search registry
- **REQ-CLI-004**: CLI shall support `mdlibs login` for authentication
- **REQ-CLI-005**: CLI shall support configuration of registry URL

#### 7.2 CLI Configuration
- **REQ-CLI-CFG-001**: CLI shall read registry configuration from `.mdlibs.toml`
- **REQ-CLI-CFG-002**: CLI shall support multiple registry configurations
- **REQ-CLI-CFG-003**: CLI shall store auth tokens securely in user config

## Non-Functional Requirements

### Performance
- **REQ-PERF-001**: API endpoints shall respond within 200ms for 95th percentile
- **REQ-PERF-002**: Search queries shall complete within 500ms for 95th percentile
- **REQ-PERF-003**: Registry shall handle 100 concurrent requests

### Reliability
- **REQ-REL-001**: Registry shall have 99.9% uptime (configurable for enterprise)
- **REQ-REL-002**: Database operations shall be transactional
- **REQ-REL-003**: Failed uploads shall be cleanly rolled back

### Scalability
- **REQ-SCALE-001**: Registry shall support at least 10,000 libraries
- **REQ-SCALE-002**: Registry shall support at least 1,000 concurrent users
- **REQ-SCALE-003**: Storage backend shall be horizontally scalable

### Usability
- **REQ-USE-001**: API shall follow REST conventions
- **REQ-USE-002**: Error messages shall be clear and actionable
- **REQ-USE-003**: API documentation shall be comprehensive and up-to-date

## Priority Classification

### MVP (Must Have)
- Library publishing, search, and download (REQ-PUB-*, REQ-DISC-*, REQ-DOWN-*)
- Basic authentication with API tokens (REQ-AUTH-001, REQ-AUTH-002)
- User namespaces (REQ-SCOPE-001, REQ-SCOPE-002)
- Docker deployment (REQ-DEPLOY-001, REQ-DEPLOY-002)
- CLI integration (REQ-CLI-001, REQ-CLI-003, REQ-CLI-004)

### Phase 2 (Should Have)
- Team support (REQ-TEAM-*)
- Team namespaces (REQ-SCOPE-003)
- Usage analytics (REQ-MON-001, REQ-MON-002)
- Advanced CLI features (REQ-CLI-002, REQ-CLI-005)

### Phase 3 (Nice to Have)
- External authentication integration (REQ-AUTH-004)
- Advanced analytics and reporting (REQ-MON-003, REQ-MON-004)
- Quotas and limits (REQ-LIMIT-*)
- S3 storage backend (REQ-DEPLOY-003)
- Metrics and monitoring (REQ-OPS-002)

### Future (Enhancement)
- Web UI for registry
- Library dependency management
- Automated library building/validation
- Integration with CI/CD pipelines
- Library templates marketplace
