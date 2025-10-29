/**
 * @name Float Type Usage
 * @description Detects usage of float types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/primitive-types/float
 */

import rust

from FloatLiteral floatLit
select floatLit, "Float literal found: " + floatLit.getValue()