/**
 * @name Enum Definitions
 * @description Detects enum definitions in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/class-types/enums
 */

import rust

from Enum enumDef
select enumDef, "Enum definition found: " + enumDef.getName()