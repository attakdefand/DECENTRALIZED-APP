/**
 * This query identifies withdrawal functions that don't check user limits.
 * 
 * @name DEX Withdrawal Functions
 * @kind problem
 * @problem.severity warning
 * @id rust/dex/withdrawal-functions
 */

import rust

from Function f
where 
  f.getName().toString() = "withdraw" or
  f.getName().toString() = "enable_withdraw_disabled_mode" or
  f.getName().toString() = "disable_withdraw_disabled_mode"
select f, "Function named '" + f.getName().toString() + "' may be a withdrawal function that needs limit checks"