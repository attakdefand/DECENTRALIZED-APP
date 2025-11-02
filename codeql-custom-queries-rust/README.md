# Custom CodeQL Queries for Rust

This directory contains custom CodeQL queries for analyzing the Rust codebase of the decentralized exchange project.

## Queries

1. **[dex-transfer-check.ql](file:///d:\DECENTRALIZED-APP\codeql-custom-queries-rust\dex-transfer-check.ql)** - Identifies functions that may be related to fund transfers
2. **[dex-withdrawal-check.ql](file:///d:\DECENTRALIZED-APP\codeql-custom-queries-rust\dex-withdrawal-check.ql)** - Identifies functions that may be related to withdrawals
3. **[dex-bridge-security.ql](file:///d:\DECENTRALIZED-APP\codeql-custom-queries-rust\dex-bridge-security.ql)** - Identifies bridge functions that may need additional security checks
4. **[dex-session-key-check.ql](file:///d:\DECENTRALIZED-APP\codeql-custom-queries-rust\dex-session-key-check.ql)** - Identifies session key functions that may need additional security checks

## Usage

These queries are automatically run as part of the CI/CD pipeline through the GitHub Actions workflow defined in [.github/workflows/codeql-analysis.yml](file:///d:/DECENTRALIZED-APP/.github/workflows/codeql-analysis.yml).

## Adding New Queries

To add a new query:

1. Create a new `.ql` file in this directory
2. Follow the format shown in the existing queries
3. Ensure the query has proper metadata comments at the top
4. Test the query locally before committing

## Query Development Resources

- [CodeQL Documentation](https://codeql.github.com/docs/)
- [CodeQL for Rust](https://codeql.github.com/docs/codeql-language-guides/codeql-for-rust/)
- [GitHub CodeQL Action](https://github.com/github/codeql-action)