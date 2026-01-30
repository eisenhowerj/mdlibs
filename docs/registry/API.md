# mdlibs-Registry API Specification v1.0

## Base URL

```
https://registry.example.com/api/v1
```

## Authentication

All authenticated endpoints require an API token in the `Authorization` header:

```
Authorization: Bearer <token>
```

## Common Response Codes

- `200 OK` - Request succeeded
- `201 Created` - Resource created successfully
- `204 No Content` - Request succeeded with no response body
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Missing or invalid authentication
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists
- `413 Payload Too Large` - Library file exceeds size limit
- `429 Too Many Requests` - Rate limit exceeded
- `500 Internal Server Error` - Server error

## Error Response Format

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Library name must be alphanumeric and lowercase",
    "details": {
      "field": "name",
      "value": "My-Library"
    }
  }
}
```

---

## Authentication Endpoints

### Register User

Create a new user account.

**Endpoint:** `POST /auth/register`

**Authentication:** None

**Request Body:**
```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecurePassword123!"
}
```

**Response:** `201 Created`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "johndoe",
  "email": "john@example.com",
  "created_at": "2026-01-30T10:00:00Z"
}
```

**Validation:**
- Username: 3-32 characters, alphanumeric and hyphens only
- Email: Valid email format
- Password: Minimum 8 characters

---

### Login

Authenticate and receive an access token.

**Endpoint:** `POST /auth/login`

**Authentication:** None

**Request Body:**
```json
{
  "username": "johndoe",
  "password": "SecurePassword123!"
}
```

**Response:** `200 OK`
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2026-02-30T10:00:00Z",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe"
  }
}
```

---

### Create API Token

Generate a new API token for programmatic access.

**Endpoint:** `POST /auth/tokens`

**Authentication:** Required

**Request Body:**
```json
{
  "name": "CI/CD Token",
  "expires_in_days": 90
}
```

**Response:** `201 Created`
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440000",
  "name": "CI/CD Token",
  "token": "mdl_abc123xyz789...",
  "expires_at": "2026-04-30T10:00:00Z",
  "created_at": "2026-01-30T10:00:00Z"
}
```

**Note:** The `token` field is only returned once. Store it securely.

---

### List API Tokens

List all active tokens for the current user.

**Endpoint:** `GET /auth/tokens`

**Authentication:** Required

**Response:** `200 OK`
```json
{
  "tokens": [
    {
      "id": "650e8400-e29b-41d4-a716-446655440000",
      "name": "CI/CD Token",
      "created_at": "2026-01-30T10:00:00Z",
      "expires_at": "2026-04-30T10:00:00Z",
      "last_used_at": "2026-01-30T12:00:00Z"
    }
  ]
}
```

---

### Revoke API Token

Revoke an API token.

**Endpoint:** `DELETE /auth/tokens/:id`

**Authentication:** Required

**Response:** `204 No Content`

---

### Get Current User

Get information about the authenticated user.

**Endpoint:** `GET /users/me`

**Authentication:** Required

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "johndoe",
  "email": "john@example.com",
  "created_at": "2026-01-30T10:00:00Z",
  "libraries_count": 5,
  "total_downloads": 1250
}
```

---

## Library Endpoints

### Publish Library

Publish a new library or version to the registry.

**Endpoint:** `POST /libraries`

**Authentication:** Required

**Content-Type:** `multipart/form-data`

**Form Fields:**
- `name` (string, required): Library name
- `version` (string, required): Semantic version (e.g., "1.0.0")
- `description` (string, optional): Library description
- `tags` (string, optional): Comma-separated tags
- `license` (string, optional): License identifier (e.g., "MIT")
- `file` (binary, required): Library archive (.tar.gz)

**Example:**
```bash
curl -X POST https://registry.example.com/api/v1/libraries \
  -H "Authorization: Bearer $TOKEN" \
  -F "name=my-docs" \
  -F "version=1.0.0" \
  -F "description=My documentation library" \
  -F "tags=docs,api,reference" \
  -F "license=MIT" \
  -F "file=@my-docs.tar.gz"
```

**Response:** `201 Created`
```json
{
  "id": "750e8400-e29b-41d4-a716-446655440000",
  "scope": "johndoe",
  "name": "my-docs",
  "version": "1.0.0",
  "description": "My documentation library",
  "tags": ["docs", "api", "reference"],
  "license": "MIT",
  "size": 102400,
  "checksum": "sha256:abc123...",
  "created_at": "2026-01-30T10:00:00Z",
  "created_by": "johndoe",
  "download_url": "/api/v1/libraries/johndoe/my-docs/1.0.0/download"
}
```

**Validation:**
- Name: 3-64 characters, lowercase alphanumeric and hyphens
- Version: Valid semantic version
- File size: Maximum 100MB (configurable)
- File format: Valid .tar.gz archive

---

### Search Libraries

Search and list available libraries.

**Endpoint:** `GET /libraries`

**Authentication:** Optional (public libraries only without auth)

**Query Parameters:**
- `q` (string, optional): Search query (searches name, description, tags)
- `tag` (string, optional): Filter by specific tag
- `scope` (string, optional): Filter by scope (user or team name)
- `page` (integer, optional): Page number (default: 1)
- `per_page` (integer, optional): Results per page (default: 20, max: 100)
- `sort` (string, optional): Sort by `name`, `downloads`, `updated_at` (default: `name`)
- `order` (string, optional): Sort order `asc` or `desc` (default: `asc`)

**Example:**
```
GET /libraries?q=documentation&tag=api&page=1&per_page=20&sort=downloads&order=desc
```

**Response:** `200 OK`
```json
{
  "libraries": [
    {
      "scope": "johndoe",
      "name": "my-docs",
      "latest_version": "1.0.0",
      "description": "My documentation library",
      "tags": ["docs", "api", "reference"],
      "license": "MIT",
      "downloads": 150,
      "created_at": "2026-01-30T10:00:00Z",
      "updated_at": "2026-01-30T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

---

### Get Library Details

Get detailed information about a specific library.

**Endpoint:** `GET /libraries/:scope/:name`

**Authentication:** Optional (required for private libraries)

**Path Parameters:**
- `scope`: User or team name
- `name`: Library name

**Example:**
```
GET /libraries/johndoe/my-docs
```

**Response:** `200 OK`
```json
{
  "scope": "johndoe",
  "name": "my-docs",
  "description": "My documentation library",
  "tags": ["docs", "api", "reference"],
  "license": "MIT",
  "created_at": "2026-01-30T10:00:00Z",
  "updated_at": "2026-01-30T10:00:00Z",
  "versions": [
    {
      "version": "1.0.0",
      "size": 102400,
      "created_at": "2026-01-30T10:00:00Z",
      "downloads": 150,
      "download_url": "/api/v1/libraries/johndoe/my-docs/1.0.0/download"
    }
  ],
  "total_downloads": 150
}
```

---

### Get Library Version Details

Get details about a specific version of a library.

**Endpoint:** `GET /libraries/:scope/:name/:version`

**Authentication:** Optional (required for private libraries)

**Response:** `200 OK`
```json
{
  "id": "750e8400-e29b-41d4-a716-446655440000",
  "scope": "johndoe",
  "name": "my-docs",
  "version": "1.0.0",
  "description": "My documentation library",
  "tags": ["docs", "api", "reference"],
  "license": "MIT",
  "size": 102400,
  "checksum": "sha256:abc123...",
  "created_at": "2026-01-30T10:00:00Z",
  "created_by": "johndoe",
  "downloads": 150,
  "download_url": "/api/v1/libraries/johndoe/my-docs/1.0.0/download"
}
```

---

### Download Library

Download a specific version of a library.

**Endpoint:** `GET /libraries/:scope/:name/:version/download`

**Authentication:** Optional (required for private libraries)

**Response:** `200 OK`

**Content-Type:** `application/gzip`

**Headers:**
- `Content-Disposition: attachment; filename="my-docs-1.0.0.tar.gz"`
- `Content-Length: 102400`

**Body:** Binary tar.gz archive

---

### Delete Library Version

Delete a specific version of a library.

**Endpoint:** `DELETE /libraries/:scope/:name/:version`

**Authentication:** Required (must be owner or team admin)

**Response:** `204 No Content`

---

### List Library Versions

List all versions of a library.

**Endpoint:** `GET /libraries/:scope/:name/versions`

**Authentication:** Optional (required for private libraries)

**Response:** `200 OK`
```json
{
  "scope": "johndoe",
  "name": "my-docs",
  "versions": [
    {
      "version": "1.0.0",
      "created_at": "2026-01-30T10:00:00Z",
      "size": 102400,
      "downloads": 150
    },
    {
      "version": "0.9.0",
      "created_at": "2026-01-20T10:00:00Z",
      "size": 98304,
      "downloads": 75
    }
  ]
}
```

---

## Team Endpoints

### Create Team

Create a new team.

**Endpoint:** `POST /teams`

**Authentication:** Required

**Request Body:**
```json
{
  "name": "acme",
  "display_name": "Acme Corporation"
}
```

**Response:** `201 Created`
```json
{
  "id": "850e8400-e29b-41d4-a716-446655440000",
  "name": "acme",
  "display_name": "Acme Corporation",
  "created_at": "2026-01-30T10:00:00Z",
  "members_count": 1,
  "libraries_count": 0
}
```

---

### List Teams

List teams the current user is a member of.

**Endpoint:** `GET /teams`

**Authentication:** Required

**Response:** `200 OK`
```json
{
  "teams": [
    {
      "id": "850e8400-e29b-41d4-a716-446655440000",
      "name": "acme",
      "display_name": "Acme Corporation",
      "role": "admin",
      "members_count": 5,
      "libraries_count": 12
    }
  ]
}
```

---

### Get Team Details

Get detailed information about a team.

**Endpoint:** `GET /teams/:id`

**Authentication:** Required (must be team member)

**Response:** `200 OK`
```json
{
  "id": "850e8400-e29b-41d4-a716-446655440000",
  "name": "acme",
  "display_name": "Acme Corporation",
  "created_at": "2026-01-30T10:00:00Z",
  "members_count": 5,
  "libraries_count": 12,
  "total_downloads": 5000
}
```

---

### Add Team Member

Add a user to a team.

**Endpoint:** `POST /teams/:id/members`

**Authentication:** Required (must be team admin)

**Request Body:**
```json
{
  "username": "janedoe",
  "role": "member"
}
```

**Response:** `201 Created`
```json
{
  "user_id": "950e8400-e29b-41d4-a716-446655440000",
  "username": "janedoe",
  "role": "member",
  "joined_at": "2026-01-30T10:00:00Z"
}
```

---

### List Team Members

List all members of a team.

**Endpoint:** `GET /teams/:id/members`

**Authentication:** Required (must be team member)

**Response:** `200 OK`
```json
{
  "members": [
    {
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "johndoe",
      "role": "admin",
      "joined_at": "2026-01-30T10:00:00Z"
    },
    {
      "user_id": "950e8400-e29b-41d4-a716-446655440000",
      "username": "janedoe",
      "role": "member",
      "joined_at": "2026-01-30T11:00:00Z"
    }
  ]
}
```

---

### Remove Team Member

Remove a user from a team.

**Endpoint:** `DELETE /teams/:id/members/:user_id`

**Authentication:** Required (must be team admin)

**Response:** `204 No Content`

---

## Statistics Endpoints

### Get Download Statistics

Get download statistics for libraries.

**Endpoint:** `GET /stats/downloads`

**Authentication:** Required

**Query Parameters:**
- `scope` (string, optional): Filter by scope
- `library` (string, optional): Filter by library name
- `days` (integer, optional): Number of days to include (default: 30)

**Response:** `200 OK`
```json
{
  "total_downloads": 5000,
  "downloads_by_day": [
    {
      "date": "2026-01-30",
      "count": 150
    },
    {
      "date": "2026-01-29",
      "count": 145
    }
  ],
  "top_libraries": [
    {
      "scope": "johndoe",
      "name": "my-docs",
      "downloads": 150
    }
  ]
}
```

---

### Get Library Statistics

Get statistics about libraries in the registry.

**Endpoint:** `GET /stats/libraries`

**Authentication:** Required

**Response:** `200 OK`
```json
{
  "total_libraries": 120,
  "total_versions": 450,
  "total_size_bytes": 5242880000,
  "libraries_by_tag": [
    {
      "tag": "documentation",
      "count": 45
    },
    {
      "tag": "api",
      "count": 32
    }
  ]
}
```

---

## System Endpoints

### Health Check

Check if the registry is healthy.

**Endpoint:** `GET /health`

**Authentication:** None

**Response:** `200 OK`
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "checks": {
    "database": "ok",
    "storage": "ok"
  },
  "timestamp": "2026-01-30T10:00:00Z"
}
```

**Response:** `503 Service Unavailable` (if unhealthy)
```json
{
  "status": "unhealthy",
  "version": "1.0.0",
  "checks": {
    "database": "error",
    "storage": "ok"
  },
  "timestamp": "2026-01-30T10:00:00Z"
}
```

---

### Metrics

Get Prometheus-compatible metrics.

**Endpoint:** `GET /metrics`

**Authentication:** None (can be configured to require auth)

**Response:** `200 OK`

**Content-Type:** `text/plain`

```
# HELP mdlibs_registry_libraries_total Total number of libraries
# TYPE mdlibs_registry_libraries_total gauge
mdlibs_registry_libraries_total 120

# HELP mdlibs_registry_downloads_total Total number of downloads
# TYPE mdlibs_registry_downloads_total counter
mdlibs_registry_downloads_total 5000

# HELP mdlibs_registry_requests_total Total HTTP requests
# TYPE mdlibs_registry_requests_total counter
mdlibs_registry_requests_total{method="GET",endpoint="/libraries",status="200"} 1500
```

---

## Rate Limiting

The registry implements rate limiting to prevent abuse:

- **Anonymous requests**: 60 requests per hour
- **Authenticated requests**: 5000 requests per hour
- **Publishing**: 100 uploads per day per user/team

Rate limit information is included in response headers:

```
X-RateLimit-Limit: 5000
X-RateLimit-Remaining: 4999
X-RateLimit-Reset: 1738233600
```

When rate limit is exceeded, the API returns `429 Too Many Requests`:

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Try again in 3600 seconds.",
    "retry_after": 3600
  }
}
```

---

## Pagination

List endpoints support pagination with the following parameters:

- `page`: Page number (1-indexed)
- `per_page`: Items per page (default: 20, max: 100)

Pagination metadata is included in responses:

```json
{
  "items": [...],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 150,
    "total_pages": 8
  }
}
```

---

## Versioning

The API is versioned via the URL path (`/api/v1`). Breaking changes will result in a new version (`/api/v2`). Non-breaking changes may be added to existing versions.

---

## CORS

The registry supports CORS for web-based clients. Configure allowed origins in the server configuration.

---

## Client Libraries

Official client libraries (future):
- Rust (integrated in mdlibs CLI)
- Python
- JavaScript/TypeScript
- Go
