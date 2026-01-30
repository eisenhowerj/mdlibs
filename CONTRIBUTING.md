# Contributing to mdlibs

Thank you for your interest in contributing to mdlibs! This document provides guidelines for contributing to the project.

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive community.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/mdlibs.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.56 or higher (for Rust 2021 edition)
- Cargo (comes with Rust)

### Building the Project

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

Before submitting a pull request, please format your code:

```bash
cargo fmt
```

### Linting

Run Clippy to check for common mistakes:

```bash
cargo clippy -- -D warnings
```

## Pull Request Guidelines

- Keep pull requests focused on a single feature or bug fix
- Write clear, descriptive commit messages
- Update documentation as needed
- Add tests for new functionality
- Ensure all tests pass before submitting
- Follow the existing code style

## Reporting Issues

When reporting issues, please include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Any relevant error messages or logs

## Feature Requests

We welcome feature requests! Please open an issue and describe:

- The problem you're trying to solve
- Your proposed solution
- Any alternatives you've considered

## Questions?

If you have questions, please open an issue with the "question" label.

## License

By contributing to mdlibs, you agree that your contributions will be licensed under the MIT License.
