/**
 * @name String Type Usage
 * @description Detects usage of string types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/primitive-types/string
 */

import rust

from StringLiteral strLit
select strLit, "String literal found: " + strLit.getValue()