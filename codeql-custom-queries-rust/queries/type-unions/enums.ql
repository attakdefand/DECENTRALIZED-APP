/**
 * @name Type Unions - Enum Patterns
 * @description Detects enum patterns that act as type unions in Rust
 * @kind problem
 * @problem.severity warning
 * @id rust/type-unions/enums
 */

import rust

// Look for enums with different variant types (union-like behavior)
from Enum enumDef
where enumDef.getAVariant().getNumFields() > 0
select enumDef, "Type union pattern detected through enum: " + enumDef.getName()