/**
 * @name Date Type Usage
 * @description Detects usage of date/time types in Rust code
 * @kind problem
 * @problem.severity warning
 * @id rust/primitive-types/date
 */

import rust

// Look for chrono library usage (most common date/time library in Rust)
from CallExpr call, Function func
where
  func.hasName("Utc::now") or
  func.hasName("Local::now") or
  func.hasName("NaiveDateTime::from_timestamp") or
  func.hasName("DateTime::from_utc")
select call, "Date/time function call found: " + func.getName()