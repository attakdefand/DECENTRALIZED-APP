/**
 * @name Trait Definitions
 * @description Detects trait definitions in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/class-types/traits
 */

import rust

from Trait traitDef
select traitDef, "Trait definition found: " + traitDef.getName()