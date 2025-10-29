/**
 * @name Boolean Type Usage
 * @description Detects usage of boolean types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/primitive-types/boolean
 */

import rust

from BooleanLiteral boolLit
select boolLit, "Boolean literal found: " + boolLit.getValue()