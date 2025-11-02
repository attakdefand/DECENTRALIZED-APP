/**
 * This query identifies bridge functions that may need additional security checks.
 * 
 * @name DEX Bridge Security Functions
 * @kind problem
 * @problem.severity warning
 * @id rust/dex/bridge-security-functions
 */

import rust

from Function f
where 
  f.getName().toString() = "submit_challenge" or
  f.getName().toString() = "resolve_challenge" or
  f.getName().toString() = "verify_proof"
select f, "Function named '" + f.getName().toString() + "' is a bridge security function that needs thorough validation"