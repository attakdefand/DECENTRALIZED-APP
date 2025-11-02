# Contributing to DECENTRALIZED-APP

Thank you for your interest in contributing to DECENTRALIZED-APP! We welcome contributions from the community and appreciate your efforts to improve our project.

## Code of Conduct

Please read and follow our [Code of Conduct](GUIDELINE-ROLE-DAPP.MD) to ensure a welcoming and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

If you find a bug in the project, please create an issue on GitHub with the following information:

- A clear and descriptive title
- A detailed description of the problem
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Any relevant logs or error messages
- Your environment information (OS, Rust version, etc.)

### Suggesting Enhancements

We welcome ideas for new features or improvements to existing functionality. Please create an issue on GitHub with:

- A clear and descriptive title
- A detailed explanation of the proposed enhancement
- The motivation for the enhancement
- Any potential implementation details

### Code Contributions

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes
4. Add or update tests as necessary
5. Ensure all tests pass
6. Update documentation if needed
7. Submit a pull request

#### Pull Request Guidelines

- Keep pull requests focused on a single feature or bug fix
- Write clear commit messages
- Include tests for new functionality
- Update documentation when necessary
- Follow the existing code style
- Reference any related issues in your pull request description

## Development Setup

1. Install Rust nightly toolchain (required for edition2024 support)
   ```bash
   rustup install nightly
   rustup default nightly
   ```
2. Install Node.js (for development tools)
3. Install Docker (for containerized services)
4. Clone the repository
5. Run `cargo build --workspace` to build all components

## Code Style

- Follow Rust naming conventions
- Use `rustfmt` to format your code
- Run `clippy` to catch common mistakes and improve code quality
- Write clear, concise comments
- Document public APIs with rustdoc

## Testing

- Write unit tests for new functionality
- Ensure all existing tests pass before submitting a pull request
- Run the full test suite with `cargo test --workspace`

## Security

If you discover a security vulnerability, please follow our [security policy](SECURITY.md) rather than creating a public issue.

## Questions?

If you have any questions about contributing, feel free to ask in the GitHub issues or contact the maintainers directly.

Thank you for contributing to DECENTRALIZED-APP!