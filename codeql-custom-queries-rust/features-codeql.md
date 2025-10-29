# Main Types in CodeQL

CodeQL is statically typed, and its types fall into several categories:

## 1. Primitive Types
These are built-in and always available:

- boolean: true, false
- int: 32-bit integers
- float: 64-bit floating-point numbers
- string: Finite strings of 16-bit characters
- date: Dates and optionally times

✅ **Implemented**: See [queries/primitive-types/](queries/primitive-types/) directory

## 2. Character Types
Used for string manipulation and character-level operations.

✅ **Implemented**: See [queries/character-types/](queries/character-types/) directory

## 3. Class Types
Represent entities in the database (e.g., Expr, Function, Variable)

Can be extended and queried using predicates

✅ **Implemented**: See [queries/class-types/](queries/class-types/) directory

## 4. Class Domain Types
Specialized class types tied to a specific domain (e.g., Java, C++, Python)

Examples: JavaExpr, CppFunction, PythonModule

✅ **Implemented**: See [queries/class-domain-types/](queries/class-domain-types/) directory

## 5. Algebraic Data Types
Custom types defined using abstract and extends

Useful for modeling complex hierarchies

✅ **Implemented**: See [queries/algebraic-data-types/](queries/algebraic-data-types/) directory

## 6. Type Unions
Allow variables to hold values of multiple types

Example: Expr + Stmt means a variable can be either an expression or a statement

✅ **Implemented**: See [queries/type-unions/](queries/type-unions/) directory

## 7. Database Types
Types that represent elements from the code database

Examples: Element, Location, File

✅ **Implemented**: See [queries/database-types/](queries/database-types/) directory

## Testing

All features have been implemented with corresponding CodeQL queries and test cases.
See the [test-project/](test-project/) directory for a comprehensive test suite.