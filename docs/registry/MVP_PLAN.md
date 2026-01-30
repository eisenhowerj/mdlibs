# mdlibs-Registry MVP Plan

## Executive Summary

This document outlines the Minimum Viable Product (MVP) plan for the mdlibs-Registry, a self-hosted service for publishing and discovering markdown libraries. The MVP focuses on core functionality needed for enterprise/corporate usage while maintaining a clear path for future enhancements.

## MVP Scope

### Goals
1. Enable teams to publish and share markdown libraries internally
2. Provide secure, authenticated access to libraries
3. Support easy deployment in corporate environments
4. Integrate seamlessly with existing mdlibs CLI
5. Provide foundation for future enterprise features

### Non-Goals (Post-MVP)
- Web UI for registry management
- OAuth2/OIDC integration
- S3-compatible storage backend
- Advanced analytics and reporting
- Library dependency resolution
- CI/CD pipeline integration

## Feature Prioritization

### Must Have (MVP v1.0)

#### 1. Core Library Management
- **Priority**: P0 (Critical)
- **Effort**: 3 weeks
- **Features**:
  - Publish markdown libraries as tar.gz archives
  - Download libraries by name and version
  - Semantic versioning support
  - Library metadata (name, version, description, tags, license)
  - List and search libraries
  - Delete library versions (owner only)

**Rationale**: Core functionality required for any registry.

#### 2. User Authentication & Authorization
- **Priority**: P0 (Critical)
- **Effort**: 2 weeks
- **Features**:
  - User registration and login
  - API token generation and management
  - Token-based authentication for API requests
  - User-scoped libraries (@username/library)
  - Basic authorization (owners can modify their libraries)

**Rationale**: Essential for secure corporate usage.

#### 3. Basic Search & Discovery
- **Priority**: P0 (Critical)
- **Effort**: 1 week
- **Features**:
  - Search libraries by name
  - Search libraries by description and tags
  - Paginated results
  - Sort by name, downloads, or date

**Rationale**: Users need to discover available libraries.

#### 4. CLI Integration
- **Priority**: P0 (Critical)
- **Effort**: 2 weeks
- **Features**:
  - `mdlibs login` - Authenticate with registry
  - `mdlibs publish` - Publish library to registry
  - `mdlibs search` - Search registry
  - Registry configuration in `.mdlibs.toml`
  - Secure token storage

**Rationale**: Seamless integration with existing workflow.

#### 5. Docker Deployment
- **Priority**: P0 (Critical)
- **Effort**: 1 week
- **Features**:
  - Dockerfile for registry server
  - docker-compose.yml with PostgreSQL
  - SQLite support for simple deployments
  - Environment-based configuration
  - Volume mounts for storage and data
  - Health check endpoint

**Rationale**: Easy deployment is critical for adoption.

#### 6. Basic Documentation
- **Priority**: P0 (Critical)
- **Effort**: 1 week
- **Features**:
  - Deployment guide
  - CLI usage guide
  - API documentation
  - Configuration reference
  - Troubleshooting guide

**Rationale**: Documentation is essential for self-hosted software.

### Should Have (MVP v1.0)

#### 7. Usage Tracking
- **Priority**: P1 (High)
- **Effort**: 3 days
- **Features**:
  - Track download counts per library
  - Track download counts per version
  - Record download timestamps
  - Basic usage statistics endpoint

**Rationale**: Useful for understanding library adoption.

#### 8. Security Features
- **Priority**: P1 (High)
- **Effort**: 1 week
- **Features**:
  - Password hashing (Argon2)
  - Token hashing in database
  - File size limits
  - Rate limiting on API endpoints
  - Input validation and sanitization
  - Audit logging for security events

**Rationale**: Security is crucial for enterprise deployment.

### Could Have (Post-MVP)

#### 9. Team Support
- **Priority**: P2 (Medium)
- **Effort**: 2 weeks
- **Features**:
  - Create and manage teams
  - Team-scoped libraries (@team/library)
  - Team member management
  - Team admin roles

**Rationale**: Important for larger organizations but not critical for MVP.

#### 10. Advanced Analytics
- **Priority**: P2 (Medium)
- **Effort**: 1 week
- **Features**:
  - Download trends over time
  - Popular libraries dashboard
  - User activity reports
  - Storage usage reports

**Rationale**: Nice to have but not critical initially.

## MVP Architecture

### Technology Stack

#### Backend (Registry Server)
- **Language**: Rust 1.74+
- **Web Framework**: Actix-web 4.x
- **Database ORM**: SQLx (supports both PostgreSQL and SQLite)
- **Authentication**: JWT tokens or API keys
- **Serialization**: Serde with JSON

#### Database
- **Primary**: PostgreSQL 15+ (recommended for production)
- **Alternative**: SQLite 3.x (development/small deployments)

#### Storage
- **MVP**: File system with structured directories
- **Future**: S3-compatible storage

#### CLI Extension
- **Existing**: mdlibs CLI (Rust)
- **New Dependencies**: 
  - `reqwest` for HTTP client
  - `tokio` for async runtime
  - `dirs` for config directory

### File Structure

```
mdlibs/
├── registry/                    # New registry server package
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── .dockerignore
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/                # API handlers
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── libraries.rs
│   │   │   └── health.rs
│   │   ├── db/                 # Database layer
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   └── schema.rs
│   │   ├── storage/            # Storage layer
│   │   │   ├── mod.rs
│   │   │   └── filesystem.rs
│   │   ├── auth/               # Authentication
│   │   │   ├── mod.rs
│   │   │   └── tokens.rs
│   │   └── config.rs           # Configuration
│   ├── migrations/             # Database migrations
│   └── tests/
├── src/                         # Existing CLI
│   ├── commands/
│   │   ├── login.rs            # NEW
│   │   └── publish.rs          # NEW
│   └── registry/               # NEW: Registry client
│       ├── mod.rs
│       └── client.rs
├── docs/
│   └── registry/               # Registry documentation
│       ├── REQUIREMENTS.md
│       ├── ARCHITECTURE.md
│       ├── API.md
│       ├── DEPLOYMENT.md       # NEW
│       └── CLI_INTEGRATION.md  # NEW
└── docker-compose.yml          # NEW
```

## Implementation Phases

### Phase 1: Foundation (Week 1)
**Goal**: Set up project structure and core infrastructure

**Tasks**:
1. Create registry server Rust project
2. Set up Actix-web with basic routing
3. Implement database schema and migrations
4. Set up SQLx with PostgreSQL and SQLite support
5. Create configuration system (env vars + config file)
6. Implement health check endpoint
7. Write basic integration tests

**Deliverables**:
- Registry server skeleton
- Database schema
- Basic health endpoint
- Docker setup

### Phase 2: Authentication (Week 2)
**Goal**: Implement user management and authentication

**Tasks**:
1. Implement user registration endpoint
2. Implement login endpoint
3. Implement password hashing (Argon2)
4. Implement API token generation
5. Implement token middleware for authentication
6. Implement token management endpoints (list, revoke)
7. Add authentication tests

**Deliverables**:
- User registration and login
- API token system
- Authentication middleware

### Phase 3: Library Management (Week 3-4)
**Goal**: Implement core library operations

**Tasks**:
1. Implement file storage layer (filesystem)
2. Implement library publish endpoint
3. Implement library download endpoint
4. Implement library search/list endpoint
5. Implement library details endpoint
6. Implement library delete endpoint
7. Add library validation and checksums
8. Add comprehensive tests

**Deliverables**:
- Complete library CRUD operations
- File storage system
- Library validation

### Phase 4: CLI Integration (Week 5)
**Goal**: Extend CLI to work with registry

**Tasks**:
1. Add registry client module to CLI
2. Implement `mdlibs login` command
3. Implement `mdlibs publish` command
4. Update `mdlibs search` to support registry
5. Implement token storage in user config
6. Add registry configuration support
7. Add CLI tests

**Deliverables**:
- CLI can authenticate with registry
- CLI can publish libraries
- CLI can search registry

### Phase 5: Docker & Deployment (Week 6)
**Goal**: Make registry easily deployable

**Tasks**:
1. Create optimized Dockerfile
2. Create docker-compose.yml
3. Add environment variable configuration
4. Create deployment documentation
5. Create sample configuration files
6. Test deployment on clean system

**Deliverables**:
- Production-ready Docker image
- docker-compose deployment
- Deployment documentation

### Phase 6: Security & Polish (Week 7)
**Goal**: Harden security and improve UX

**Tasks**:
1. Implement rate limiting
2. Add input validation everywhere
3. Implement audit logging
4. Add file size limits
5. Security audit and penetration testing
6. Performance optimization
7. Error message improvements

**Deliverables**:
- Security-hardened registry
- Audit logging
- Rate limiting

### Phase 7: Documentation & Testing (Week 8)
**Goal**: Comprehensive documentation and testing

**Tasks**:
1. Write deployment guide
2. Write API documentation
3. Write CLI integration guide
4. Write troubleshooting guide
5. Create example workflows
6. Integration testing
7. Load testing (basic)
8. User acceptance testing

**Deliverables**:
- Complete documentation
- Test coverage > 80%
- Performance benchmarks

## Success Metrics

### MVP Launch Criteria
- [ ] Users can register and authenticate
- [ ] Users can publish libraries via CLI
- [ ] Users can search and download libraries via CLI
- [ ] Registry can be deployed with docker-compose in < 10 minutes
- [ ] API response times < 200ms (95th percentile)
- [ ] Test coverage > 80%
- [ ] Zero critical security vulnerabilities
- [ ] Documentation covers all common use cases

### KPIs (Post-Launch)
- Time to first library publish: < 5 minutes
- Average API response time: < 100ms
- Registry uptime: > 99.9%
- User satisfaction: > 4.5/5
- Documentation completeness: > 90%

## Risk Assessment

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Database performance issues | High | Medium | Use connection pooling, indexes, and implement caching |
| Storage scalability | Medium | Low | Design storage layer to be swappable, plan S3 support |
| API security vulnerabilities | High | Medium | Security audit, input validation, penetration testing |
| Docker deployment complexity | Medium | Low | Thorough testing, clear documentation |

### Resource Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Scope creep | High | High | Strict MVP scope, defer features to post-MVP |
| Timeline slippage | Medium | Medium | Weekly progress reviews, adjust scope if needed |
| Testing time underestimated | Medium | Medium | Allocate full week for testing and polish |

## Post-MVP Roadmap

### Version 1.1 (+ 2 weeks)
- Team support
- Team-scoped libraries
- Enhanced search (full-text)

### Version 1.2 (+ 2 weeks)
- Advanced analytics dashboard
- Usage quotas and limits
- Web UI for browsing

### Version 2.0 (+ 4 weeks)
- OAuth2/OIDC integration
- S3-compatible storage backend
- Library dependency management
- Automated security scanning

## Resource Requirements

### Development Team
- 1 Backend Developer (full-time, 8 weeks)
- 1 DevOps Engineer (part-time, 2 weeks)
- 1 Technical Writer (part-time, 1 week)
- 1 QA Engineer (part-time, 1 week)

### Infrastructure (Development)
- PostgreSQL database (local or cloud)
- File storage (local filesystem)
- CI/CD pipeline (GitHub Actions)
- Docker registry for images

### Infrastructure (Testing)
- Test server for integration tests
- Load testing tools (k6 or similar)
- Security scanning tools

## Dependencies

### External Dependencies
- PostgreSQL 15+ or SQLite 3.x
- Rust 1.74+
- Docker 20+
- Docker Compose 2+

### Rust Crates
- `actix-web` - Web framework
- `sqlx` - Database ORM
- `tokio` - Async runtime
- `serde` - Serialization
- `argon2` - Password hashing
- `jsonwebtoken` - JWT tokens
- `reqwest` - HTTP client (CLI)
- `tar` - Archive handling
- `flate2` - Gzip compression

## Launch Plan

### Pre-Launch (1 week before)
- [ ] Complete all MVP features
- [ ] Pass all security checks
- [ ] Complete documentation
- [ ] Perform load testing
- [ ] Deploy to staging environment
- [ ] Internal beta testing

### Launch Day
- [ ] Deploy to production
- [ ] Publish Docker image to registry
- [ ] Release CLI version with registry support
- [ ] Publish documentation
- [ ] Announce to early adopters

### Post-Launch (First week)
- [ ] Monitor for issues
- [ ] Respond to user feedback
- [ ] Fix critical bugs within 24 hours
- [ ] Gather usage metrics
- [ ] Plan iteration based on feedback

## Success Criteria

The MVP is considered successful if:

1. **Functional**: All P0 features working correctly
2. **Reliable**: < 5 bugs per week reported
3. **Performant**: API response times < 200ms (95th percentile)
4. **Secure**: Zero critical security issues
5. **Usable**: Users can publish first library in < 5 minutes
6. **Deployable**: Can be deployed in < 10 minutes with docker-compose
7. **Documented**: All features have documentation
8. **Adopted**: At least 5 early adopter teams using it

## Conclusion

This MVP plan provides a focused, achievable path to launching a self-hosted mdlibs-Registry. By concentrating on core features and enterprise requirements, we can deliver value quickly while establishing a foundation for future enhancements.

The 8-week timeline is realistic with a dedicated developer, and the phased approach allows for iterative refinement based on testing and feedback. Post-MVP features are well-defined, making it easy to plan future iterations based on user needs and feedback.
