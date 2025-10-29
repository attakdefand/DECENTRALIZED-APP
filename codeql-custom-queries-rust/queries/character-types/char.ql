/**
 * @name Character Type Usage
 * @description Detects usage of character types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/character-types/char
 */

import rust

from CharLiteral charLit
select charLit, "Character literal found: " + charLit.getValue()