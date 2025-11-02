/**
 * This query identifies session key functions that may need additional security checks.
 * @name DEX Session Key Functions
 * @kind problem
 * @problem.severity warning
 * @id rust/dex/session-key-functions
 */

import rust

from Function f
where 
  f.getName().toString() = "create_session_key" or
  f.getName().toString() = "validate_session_key" or
  f.getName().toString() = "revoke_session_key" or
  f.getName().toString() = "use_session_key"
select f, "Function named '" + f.getName().toString() + "' is a session key function that needs thorough validation"