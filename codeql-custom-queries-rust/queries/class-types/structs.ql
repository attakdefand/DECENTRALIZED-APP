/**
 * @name Struct Definitions
 * @description Detects struct definitions in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/class-types/structs
 */

import rust

from Struct structDef
select structDef, "Struct definition found: " + structDef.getName()