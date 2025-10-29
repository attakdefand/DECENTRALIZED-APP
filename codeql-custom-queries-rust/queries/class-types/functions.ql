/**
 * @name Function Definitions
 * @description Detects function definitions in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/class-types/functions
 */

import rust

from Function funcDef
select funcDef, "Function definition found: " + funcDef.getName()