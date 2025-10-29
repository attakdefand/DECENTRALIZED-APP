/**
 * @name Algebraic Data Types - Enums
 * @description Detects algebraic data types implemented as enums in Rust
 * @kind problem
 * @problem.severity warning
 * @id rust/algebraic-data-types/enums
 */

import rust

// Look for enums with fields (algebraic data types)
from EnumVariant variant
where variant.getNumFields() > 0
select variant, "Algebraic data type variant found: " + variant.getName() + " with " + variant.getNumFields() + " fields"