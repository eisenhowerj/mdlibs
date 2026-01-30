# Getting Started with mdlibs-Registry

## Overview

Welcome to the mdlibs-Registry! This guide will help you get started with the self-hosted registry for markdown libraries.

## What is mdlibs-Registry?

The mdlibs-Registry is a self-hosted service that enables teams to:

- **Publish** markdown libraries to a centralized repository
- **Discover** libraries through search
- **Share** documentation and templates across teams
- **Track** usage and downloads
- **Collaborate** with team-based namespaces

Think of it as a private package registry, but specifically designed for markdown documentation and libraries.

## Current Status

**Phase**: Design Complete ✅  
**Next**: Implementation (8-week MVP plan)

The design and architecture phase is complete with comprehensive documentation. The actual server implementation is scheduled according to the MVP plan documented in [MVP_PLAN.md](MVP_PLAN.md).

## Quick Tour of Documentation

### 1. Start Here: Requirements
**File**: [REQUIREMENTS.md](REQUIREMENTS.md)

Understand what the registry will do:
- 80+ detailed functional requirements
- Non-functional requirements (performance, security, scalability)
- Priority classification (MVP, Phase 2, Future)

**Recommended reading time**: 15-20 minutes

### 2. Understand the Design: Architecture
**File**: [ARCHITECTURE.md](ARCHITECTURE.md)

Deep dive into how it works:
- System architecture diagrams
- Database schema (6 tables)
- Component details
- Technology choices and rationale
- Implementation roadmap

**Recommended reading time**: 30-40 minutes

### 3. Learn the API: API Specification
**File**: [API.md](API.md)

Complete REST API reference:
- 30+ endpoints with examples
- Authentication flows
- Request/response formats
- Error handling
- Rate limiting

**Recommended reading time**: 20-30 minutes (reference document)

### 4. Plan Implementation: MVP Plan
**File**: [MVP_PLAN.md](MVP_PLAN.md)

Detailed development roadmap:
- 8-week timeline
- 7 implementation phases
- Resource requirements
- Risk assessment
- Success metrics

**Recommended reading time**: 20-25 minutes

### 5. Deploy the Registry: Deployment Guide
**File**: [DEPLOYMENT.md](DEPLOYMENT.md)

How to deploy and operate:
- Quick start with Docker Compose
- Configuration reference
- Production deployment
- Monitoring and backups
- Troubleshooting

**Recommended reading time**: 15-20 minutes for quickstart, full reference

### 6. Use with CLI: CLI Integration
**File**: [CLI_INTEGRATION.md](CLI_INTEGRATION.md)

How to use the CLI with registry:
- Installation and configuration
- Authentication
- Publishing and installing libraries
- CI/CD integration
- Complete workflow examples

**Recommended reading time**: 20-25 minutes

## For Different Audiences

### I'm a Developer Planning to Implement This

**Start with**:
1. [REQUIREMENTS.md](REQUIREMENTS.md) - Understand what to build
2. [ARCHITECTURE.md](ARCHITECTURE.md) - Understand how to build it
3. [MVP_PLAN.md](MVP_PLAN.md) - Understand the timeline and phases
4. [API.md](API.md) - Reference for API implementation

**Key files for development**:
- `registry/Cargo.toml` - Dependencies
- `registry/Dockerfile` - Container setup
- Database schema in ARCHITECTURE.md

### I'm a DevOps Engineer Planning to Deploy This

**Start with**:
1. [DEPLOYMENT.md](DEPLOYMENT.md) - Deployment options and procedures
2. [ARCHITECTURE.md](ARCHITECTURE.md) - System components and requirements
3. `docker-compose.yml` and `.env.example` - Infrastructure code

**Key concerns addressed**:
- Docker deployment ✓
- Database options (PostgreSQL/SQLite) ✓
- HTTPS/reverse proxy ✓
- Backups and monitoring ✓
- High availability ✓

### I'm a Team Lead Evaluating This Solution

**Start with**:
1. [REQUIREMENTS.md](REQUIREMENTS.md) - Features and capabilities
2. [MVP_PLAN.md](MVP_PLAN.md) - Timeline and resource requirements
3. [DEPLOYMENT.md](DEPLOYMENT.md) - Deployment complexity

**Key decision factors**:
- **Timeline**: 8-week MVP implementation
- **Resources**: 1 developer full-time + part-time support
- **Deployment**: Docker-based, 10-minute setup
- **Maintenance**: Standard database + file storage backups
- **Scalability**: Horizontal scaling ready

### I'm a User Who Will Use the CLI

**Start with**:
1. [CLI_INTEGRATION.md](CLI_INTEGRATION.md) - Complete user guide
2. [API.md](API.md) - Understanding the capabilities

**What you can do**:
- Publish your markdown libraries
- Search and discover team libraries
- Install libraries with single command
- Manage authentication tokens
- Integrate with CI/CD

## Architecture Summary

```
┌─────────────────────────────────────────┐
│           mdlibs CLI                    │
│  (publish, search, install, login)      │
└─────────────┬───────────────────────────┘
              │ HTTPS/REST API
              ▼
┌─────────────────────────────────────────┐
│      mdlibs-Registry Server             │
│  ┌─────────────────────────────────┐   │
│  │  API Layer (Actix-web)          │   │
│  │  • Auth  • Publish  • Search    │   │
│  └─────────────┬──────────────────┬┘   │
│                │                  │     │
│  ┌─────────────▼───────┐  ┌──────▼──┐  │
│  │  Database           │  │ Storage │  │
│  │  (PostgreSQL/SQLite)│  │ (Files) │  │
│  └─────────────────────┘  └─────────┘  │
└─────────────────────────────────────────┘
```

**Key Features**:
- RESTful API for all operations
- JWT/API token authentication
- Semantic versioning for libraries
- Team-based namespaces
- Usage analytics
- Docker deployment

## Technology Stack

### Server
- **Language**: Rust 1.74+
- **Web Framework**: Actix-web
- **Database**: PostgreSQL or SQLite
- **Authentication**: JWT tokens, Argon2 password hashing

### CLI
- **Language**: Rust 1.74+
- **HTTP Client**: Reqwest
- **Existing**: Already implemented for local library management

### Deployment
- **Containerization**: Docker + Docker Compose
- **Database**: PostgreSQL 15 (recommended) or SQLite (simple deployments)
- **Storage**: File system (MVP), S3-compatible (future)

## MVP Features

### Included in v1.0
✅ User registration and authentication  
✅ API token management  
✅ Library publishing (with metadata)  
✅ Library search and discovery  
✅ Library download  
✅ Download tracking  
✅ User-scoped namespaces  
✅ Docker deployment  
✅ CLI integration  

### Coming in v1.1
⏳ Team support  
⏳ Team-scoped namespaces  
⏳ Enhanced analytics  
⏳ Advanced search  

### Planned for v2.0
📋 OAuth2/OIDC integration  
📋 S3 storage backend  
📋 Web UI  
📋 Quotas and limits  

## Security Highlights

- **Passwords**: Argon2 hashing (industry standard)
- **Tokens**: SHA-256 hashed before storage
- **API**: Rate limiting (60/hour anon, 5000/hour auth)
- **Input**: Validation and sanitization on all endpoints
- **SQL**: Parameterized queries (no injection risk)
- **HTTPS**: TLS/SSL support via reverse proxy
- **Audit**: Security event logging

## Performance Targets

- **API Response**: < 200ms (95th percentile)
- **Search**: < 500ms (95th percentile)
- **Concurrent Users**: 100+
- **Libraries**: 10,000+
- **Uptime**: 99.9%

## Getting Started (Once Implemented)

### For Administrators

1. **Deploy the registry**:
```bash
cp .env.example .env
# Edit .env with your configuration
docker-compose up -d
```

2. **Verify it's running**:
```bash
curl http://localhost:8080/health
```

3. **Create first user** via API or CLI

### For Users

1. **Configure CLI**:
```bash
mdlibs login https://registry.company.com
```

2. **Publish a library**:
```bash
cd my-library
mdlibs publish
```

3. **Search and install**:
```bash
mdlibs search documentation
mdlibs install @user/library-name
```

## Development Status

### Completed ✅
- Requirements specification (80+ requirements)
- System architecture design
- REST API specification (30+ endpoints)
- Database schema design
- MVP implementation plan (8-week timeline)
- Deployment guide and Docker setup
- CLI integration guide
- Complete documentation (74+ KB)

### In Progress 🔄
- Server implementation
  - Phase 1: Foundation (Week 1)
  - Phase 2: Authentication (Week 2)
  - Phase 3-7: Feature implementation

### Planned 📋
- v1.1: Team features
- v1.2: Advanced analytics
- v2.0: Enterprise features

## Contributing

The project is in the design phase. Implementation will begin following the MVP plan. Contributions are welcome!

Areas for contribution:
- Server implementation (Rust/Actix-web)
- CLI registry integration
- Documentation improvements
- Testing and QA
- Docker/deployment improvements

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## Support and Resources

- **Documentation**: All docs in `docs/registry/`
- **Issues**: GitHub Issues for bugs and features
- **Discussions**: GitHub Discussions for questions
- **Repository**: https://github.com/eisenhowerj/mdlibs

## Next Steps

1. **Review the documentation** based on your role (see "For Different Audiences" above)
2. **Provide feedback** on the design before implementation begins
3. **Track progress** as development proceeds according to MVP plan
4. **Test early versions** when available
5. **Contribute** code, documentation, or testing

## Questions?

- **How long until it's ready?** 8 weeks for MVP (v1.0) based on current plan
- **Can I use it now?** Design is complete, implementation is next
- **Will it work with existing mdlibs CLI?** Yes, CLI will be extended
- **What about my current libraries?** They remain local, registry is optional
- **Is it secure?** Yes, designed with security best practices (see REQUIREMENTS.md)
- **Can I deploy on-premises?** Yes, that's the primary use case
- **What about cloud?** Works on any Docker-capable host

## Conclusion

The mdlibs-Registry is a well-designed, enterprise-ready solution for self-hosted markdown library management. The comprehensive design documentation provides a solid foundation for implementation. We're excited to begin development and welcome your involvement!

---

**Document Version**: 1.0  
**Last Updated**: January 30, 2026  
**Status**: Design Complete, Ready for Implementation
