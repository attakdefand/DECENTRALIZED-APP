# CodeQL Custom Queries for Rust

This directory contains custom CodeQL queries for analyzing Rust code, organized by the main types in CodeQL as described in [features-codeql.md](features-codeql.md).

## Query Organization

### Primitive Types
Queries that detect usage of primitive types in Rust:
- `boolean.ql` - Detects boolean literals
- `integer.ql` - Detects integer literals
- `float.ql` - Detects float literals
- `string.ql` - Detects string literals
- `date.ql` - Detects date/time library usage

### Character Types
Queries that detect usage of character types in Rust:
- `char.ql` - Detects character literals

### Class Types
Queries that detect various class-like definitions in Rust:
- `structs.ql` - Detects struct definitions
- `enums.ql` - Detects enum definitions
- `functions.ql` - Detects function definitions
- `traits.ql` - Detects trait definitions

### Class Domain Types
Queries that detect domain-specific types in Rust:
- `expressions.ql` - Detects various expression types
- `statements.ql` - Detects various statement types

### Algebraic Data Types
Queries that detect algebraic data types in Rust:
- `enums.ql` - Detects enums with fields (algebraic data types)
- `options.ql` - Detects Option<T> pattern usage
- `results.ql` - Detects Result<T, E> pattern usage

### Type Unions
Queries that detect union-like patterns in Rust:
- `enums.ql` - Detects enum patterns that act as type unions
- `dynamic-dispatch.ql` - Detects dynamic dispatch patterns (Box<dyn Trait>)

### Database Types
Queries that detect CodeQL database elements:
- `elements.ql` - Detects code elements
- `locations.ql` - Detects location information
- `files.ql` - Detects file information

## Usage

To use these queries:

1. Create a CodeQL database from your Rust project:
   ```
   codeql database create my-db --language=rust --source-root=/path/to/project
   ```

2. Run the queries against your database:
   ```
   codeql database analyze my-db codeql-suite.qls --format=sarif-latest --output=results.sarif
   ```

## Test Project

The `test-project` directory contains a Rust project with examples of all features detected by these queries. This can be used to verify that the queries work correctly.