/**
 * @name Database Types - Elements
 * @description Detects CodeQL database elements in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/database-types/elements
 */

import rust

// Look for any code elements
from Element elem
select elem, "Code element found: " + elem.toString()