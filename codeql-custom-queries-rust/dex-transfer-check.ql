/**
 * This query identifies functions related to fund transfers that may need audit logging.
 * 
 * @name DEX Transfer Functions
 * @kind problem
 * @problem.severity warning
 * @id rust/dex/transfer-functions
 */

import rust

from Function f
where 
  f.getName().toString() = "submit_transfer" or
  f.getName().toString() = "relay_transfer" or
  f.getName().toString() = "submit_transfer_with_proof" or
  f.getName().toString() = "add_funds"
select f, "Function named '" + f.getName().toString() + "' may be a fund transfer function that needs audit logging"