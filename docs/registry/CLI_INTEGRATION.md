# CLI Integration with mdlibs-Registry

## Overview

This guide explains how to configure and use the mdlibs CLI tool with a self-hosted registry. The CLI provides seamless integration for publishing, searching, and installing markdown libraries from your registry.

## Prerequisites

- mdlibs CLI v0.2.0 or later (includes registry support)
- Access to a mdlibs-Registry instance
- Valid user account on the registry

## Installation

### Install/Update mdlibs CLI

#### From Source
```bash
git clone https://github.com/eisenhowerj/mdlibs.git
cd mdlibs
cargo install --path .
```

#### From Cargo (when published)
```bash
cargo install mdlibs
```

### Verify Installation

```bash
mdlibs --version
```

Should show version 0.2.0 or later with registry support.

## Configuration

### Registry Configuration

The CLI reads registry configuration from two sources:

1. **Global config**: `~/.config/mdlibs/config.toml` (user-wide settings)
2. **Local config**: `.mdlibs.toml` (project-specific settings)

### Setting Up Registry

#### Option 1: Using the CLI (Recommended)

```bash
mdlibs registry add default https://registry.example.com
```

This will:
1. Add the registry to your configuration
2. Set it as the default registry
3. Save configuration to `~/.config/mdlibs/config.toml`

#### Option 2: Manual Configuration

Create or edit `~/.config/mdlibs/config.toml`:

```toml
[library]
name = "my-library"
version = "0.1.0"

[[registry]]
name = "default"
url = "https://registry.example.com"
default = true

[[registry]]
name = "company"
url = "https://registry.company.internal"
default = false
```

### Verify Configuration

```bash
mdlibs registry list
```

Output:
```
Configured registries:
* default: https://registry.example.com (default)
  company: https://registry.company.internal
```

## Authentication

### Login to Registry

```bash
mdlibs login
```

This will:
1. Prompt for username and password
2. Authenticate with the registry
3. Receive and store an API token
4. Save token securely in `~/.config/mdlibs/credentials.toml`

**Interactive prompts**:
```
Registry: https://registry.example.com
Username: johndoe
Password: ********
✓ Successfully logged in as johndoe
Token saved to ~/.config/mdlibs/credentials.toml
```

### Login to Specific Registry

```bash
mdlibs login --registry company
```

### Login with API Token

If you already have an API token:

```bash
mdlibs login --token mdl_abc123xyz789...
```

Or set the environment variable:

```bash
export MDLIBS_TOKEN=mdl_abc123xyz789...
```

### Verify Authentication

```bash
mdlibs whoami
```

Output:
```
Logged in to: https://registry.example.com
Username: johndoe
Scope: @johndoe
Token expires: 2026-04-30
```

### Logout

```bash
mdlibs logout
```

Remove token for specific registry:
```bash
mdlibs logout --registry company
```

## Publishing Libraries

### Prepare Library for Publishing

Ensure your library has proper structure:

```
my-library/
├── .mdlibs.toml
├── docs/
│   ├── README.md
│   ├── guide.md
│   └── api.md
└── templates/
    └── template.md
```

The `.mdlibs.toml` should include metadata:

```toml
[library]
name = "my-docs"
version = "1.0.0"
description = "My documentation library"
license = "MIT"
tags = ["documentation", "api", "reference"]
```

### Publish to Registry

```bash
cd my-library
mdlibs publish
```

This will:
1. Validate library structure
2. Create `.tar.gz` archive
3. Upload to registry
4. Display confirmation

**Output**:
```
Publishing my-docs v1.0.0...
✓ Library structure validated
✓ Created archive (102.4 KB)
✓ Published to @johndoe/my-docs@1.0.0
📦 https://registry.example.com/libraries/johndoe/my-docs/1.0.0
```

### Publish to Specific Registry

```bash
mdlibs publish --registry company
```

### Publish with Custom Metadata

Override metadata from command line:

```bash
mdlibs publish \
  --version 1.0.1 \
  --description "Updated documentation" \
  --tags "docs,api,v2"
```

### Dry Run

Test publishing without actually uploading:

```bash
mdlibs publish --dry-run
```

## Searching Libraries

### Basic Search

Search across all public libraries:

```bash
mdlibs search documentation
```

Output:
```
Found 3 libraries:

@johndoe/my-docs (1.0.0)
  My documentation library
  Tags: documentation, api, reference
  Downloads: 150

@acme/api-docs (2.1.0)
  API documentation templates
  Tags: documentation, api, templates
  Downloads: 450

@devteam/guides (1.5.0)
  Development guides and tutorials
  Tags: documentation, tutorial, guides
  Downloads: 320
```

### Search with Filters

```bash
# Search by tag
mdlibs search --tag api

# Search in specific scope
mdlibs search --scope johndoe

# Combine filters
mdlibs search documentation --tag api --scope acme
```

### Search Specific Registry

```bash
mdlibs search documentation --registry company
```

### Sorting Results

```bash
# Sort by downloads (most popular)
mdlibs search documentation --sort downloads

# Sort by date (newest first)
mdlibs search documentation --sort updated
```

## Installing Libraries

### Install Library

```bash
mdlibs install @johndoe/my-docs
```

This will:
1. Download latest version
2. Extract to `./libraries/@johndoe/my-docs`
3. Update local registry

**Output**:
```
Installing @johndoe/my-docs...
✓ Downloaded v1.0.0 (102.4 KB)
✓ Extracted to ./libraries/@johndoe/my-docs
✓ Installation complete
```

### Install Specific Version

```bash
mdlibs install @johndoe/my-docs@1.0.0
```

### Install to Custom Location

```bash
mdlibs install @johndoe/my-docs --path ./my-libs
```

### Install from Specific Registry

```bash
mdlibs install @acme/api-docs --registry company
```

### List Installed Libraries

```bash
mdlibs list --installed
```

Output:
```
Installed libraries:

@johndoe/my-docs@1.0.0
  Location: ./libraries/@johndoe/my-docs
  Installed: 2026-01-30

@acme/api-docs@2.1.0
  Location: ./libraries/@acme/api-docs
  Installed: 2026-01-29
```

### Update Installed Library

```bash
mdlibs update @johndoe/my-docs
```

This checks for newer versions and updates if available.

## Managing API Tokens

### Generate New Token

```bash
mdlibs token create "CI/CD Token" --expires 90
```

Output:
```
✓ Token created successfully
Token: mdl_abc123xyz789...
Name: CI/CD Token
Expires: 2026-04-30

⚠️  Save this token securely. It won't be shown again.
```

### List Tokens

```bash
mdlibs token list
```

Output:
```
API Tokens:

CI/CD Token (id: 650e8400...)
  Created: 2026-01-30
  Expires: 2026-04-30
  Last used: 2026-01-30

Dev Token (id: 750e8400...)
  Created: 2026-01-15
  Expires: Never
  Last used: 2026-01-29
```

### Revoke Token

```bash
mdlibs token revoke 650e8400-e29b-41d4-a716-446655440000
```

Or by name:
```bash
mdlibs token revoke "CI/CD Token"
```

## Advanced Usage

### Using Multiple Registries

Configure multiple registries:

```bash
mdlibs registry add work https://registry.company.com
mdlibs registry add personal https://registry.personal.com
mdlibs registry add public https://registry.mdlibs.io
```

Set default:
```bash
mdlibs registry set-default work
```

Publish to specific registry:
```bash
mdlibs publish --registry personal
```

Search across registries:
```bash
mdlibs search "api docs" --registry work --registry public
```

### Environment Variables

Override configuration with environment variables:

```bash
# Registry URL
export MDLIBS_REGISTRY_URL=https://registry.example.com

# API Token
export MDLIBS_TOKEN=mdl_abc123xyz789...

# Timeout (seconds)
export MDLIBS_TIMEOUT=30

# Disable certificate verification (for testing only!)
export MDLIBS_INSECURE=true
```

### Configuration Precedence

Configuration is loaded in this order (later overrides earlier):

1. Built-in defaults
2. Global config (`~/.config/mdlibs/config.toml`)
3. Local config (`.mdlibs.toml`)
4. Environment variables
5. Command-line flags

### CI/CD Integration

#### GitHub Actions

```yaml
name: Publish to Registry

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install mdlibs
        run: cargo install mdlibs
      
      - name: Publish to registry
        env:
          MDLIBS_TOKEN: ${{ secrets.MDLIBS_TOKEN }}
          MDLIBS_REGISTRY_URL: https://registry.example.com
        run: mdlibs publish
```

#### GitLab CI

```yaml
publish:
  stage: deploy
  only:
    - tags
  script:
    - cargo install mdlibs
    - mdlibs publish
  variables:
    MDLIBS_TOKEN: $REGISTRY_TOKEN
    MDLIBS_REGISTRY_URL: https://registry.example.com
```

#### Jenkins

```groovy
pipeline {
    agent any
    environment {
        MDLIBS_TOKEN = credentials('mdlibs-token')
        MDLIBS_REGISTRY_URL = 'https://registry.example.com'
    }
    stages {
        stage('Publish') {
            steps {
                sh 'cargo install mdlibs'
                sh 'mdlibs publish'
            }
        }
    }
}
```

## Troubleshooting

### Authentication Issues

**Problem**: `Error: Unauthorized (401)`

**Solutions**:
- Verify you're logged in: `mdlibs whoami`
- Check token hasn't expired
- Login again: `mdlibs login`
- Verify registry URL is correct

### Publishing Fails

**Problem**: `Error: Library validation failed`

**Solutions**:
- Ensure `.mdlibs.toml` exists and is valid
- Check library structure has required directories
- Verify version is valid semver
- Check library name follows naming rules (lowercase, alphanumeric, hyphens)

**Problem**: `Error: Version already exists`

**Solutions**:
- Increment version in `.mdlibs.toml`
- Delete old version first (if you have permission)
- Use different version number

### Network Issues

**Problem**: `Error: Connection timeout`

**Solutions**:
- Check registry URL is accessible
- Verify firewall/proxy settings
- Increase timeout: `mdlibs publish --timeout 60`
- Check registry is running: `curl https://registry.example.com/health`

### Certificate Issues

**Problem**: `Error: SSL certificate verify failed`

**Solutions**:
- Install proper CA certificates
- For self-signed certs in testing: `MDLIBS_INSECURE=true` (not for production!)
- Add certificate to system trust store

### Token Storage Issues

**Problem**: `Error: Failed to save token`

**Solutions**:
- Check permissions on `~/.config/mdlibs/`
- Ensure directory exists: `mkdir -p ~/.config/mdlibs`
- Manually edit `~/.config/mdlibs/credentials.toml`

## Security Best Practices

1. **Never commit tokens** to version control
   - Add `.mdlibs/credentials.toml` to `.gitignore`
   - Use environment variables in CI/CD

2. **Use separate tokens** for different purposes
   - One for personal use
   - One for CI/CD
   - Revoke tokens when no longer needed

3. **Set token expiration**
   - Short expiration for temporary access
   - Longer for automated systems

4. **Rotate tokens regularly**
   - Create new token
   - Update systems
   - Revoke old token

5. **Use HTTPS** always
   - Never use HTTP in production
   - Verify certificate

6. **Restrict token scope** (future feature)
   - Read-only tokens for CI
   - Write tokens for publishing

## Complete Example Workflow

### Initial Setup

```bash
# Install mdlibs
cargo install mdlibs

# Configure registry
mdlibs registry add company https://registry.company.com
mdlibs registry set-default company

# Login
mdlibs login
# Enter credentials when prompted
```

### Working with Libraries

```bash
# Create new library
mdlibs init my-docs
cd my-docs

# Edit configuration
cat > .mdlibs.toml << EOF
[library]
name = "my-docs"
version = "1.0.0"
description = "My documentation library"
license = "MIT"
tags = ["docs", "internal"]
EOF

# Add content
echo "# Documentation" > docs/README.md

# Publish
mdlibs publish

# Search
mdlibs search my-docs

# Install in another project
cd ../my-project
mdlibs install @johndoe/my-docs
```

## Reference

### Command Summary

| Command | Description |
|---------|-------------|
| `mdlibs login` | Authenticate with registry |
| `mdlibs logout` | Remove authentication |
| `mdlibs whoami` | Show current user |
| `mdlibs publish` | Publish library to registry |
| `mdlibs search <query>` | Search for libraries |
| `mdlibs install <lib>` | Install library |
| `mdlibs registry add <name> <url>` | Add registry |
| `mdlibs registry list` | List configured registries |
| `mdlibs token create <name>` | Create API token |
| `mdlibs token list` | List API tokens |
| `mdlibs token revoke <id>` | Revoke API token |

### Configuration Files

| File | Purpose |
|------|---------|
| `~/.config/mdlibs/config.toml` | Global configuration |
| `~/.config/mdlibs/credentials.toml` | API tokens (keep secure!) |
| `.mdlibs.toml` | Project configuration |

## Next Steps

- [Learn more about the API](API.md)
- [Deploy your own registry](DEPLOYMENT.md)
- [Understand the architecture](ARCHITECTURE.md)
- [Read the requirements](REQUIREMENTS.md)
