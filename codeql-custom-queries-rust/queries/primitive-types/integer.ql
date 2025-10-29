/**
 * @name Integer Type Usage
 * @description Detects usage of integer types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/primitive-types/integer
 */

import rust

from IntegerLiteral intLit
select intLit, "Integer literal found: " + intLit.getValue()