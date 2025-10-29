/**
 * @name Type Unions - Dynamic Dispatch
 * @description Detects dynamic dispatch patterns (Box<dyn Trait>) in Rust
 * @kind problem
 * @problem.severity warning
 * @id rust/type-unions/dynamic-dispatch
 */

import rust

// Look for dynamic dispatch patterns
from TypeExpr typeExpr
where typeExpr.toString().matches("Box<dyn %>")
select typeExpr, "Dynamic dispatch pattern found: " + typeExpr.toString()