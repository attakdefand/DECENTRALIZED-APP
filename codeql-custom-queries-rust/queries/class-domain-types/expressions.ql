/**
 * @name Rust Expressions
 * @description Detects various expression types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/class-domain-types/expressions
 */

import rust

from Expr expr
select expr, "Expression found of type: " + expr.getClass().getName()