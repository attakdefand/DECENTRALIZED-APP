/**
 * @name Rust Statements
 * @description Detects various statement types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/class-domain-types/statements
 */

import rust

from Stmt stmt
select stmt, "Statement found of type: " + stmt.getClass().getName()