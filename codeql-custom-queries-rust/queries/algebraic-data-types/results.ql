/**
 * @name Algebraic Data Types - Result Pattern
 * @description Detects usage of Result<T, E> pattern in Rust
 * @kind problem
 * @problem.severity warning
 * @id rust/algebraic-data-types/results
 */

import rust

// Look for Result type usage
from TypeExpr typeExpr
where typeExpr.toString().matches("Result<%>")
select typeExpr, "Result type usage found: " + typeExpr.toString()