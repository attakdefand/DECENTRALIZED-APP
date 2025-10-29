/**
 * @name Algebraic Data Types - Option Pattern
 * @description Detects usage of Option<T> pattern in Rust
 * @kind problem
 * @problem.severity warning
 * @id rust/algebraic-data-types/options
 */

import rust

// Look for Option type usage
from TypeExpr typeExpr
where typeExpr.toString().matches("Option<%>")
select typeExpr, "Option type usage found: " + typeExpr.toString()