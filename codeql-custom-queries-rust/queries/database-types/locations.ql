/**
 * @name Database Types - Locations
 * @description Detects location information in CodeQL database
 * @kind problem
 * @problem.severity warning
 * @id rust/database-types/locations
 */

import rust

// Look for location information
from Location loc
select loc, "Code location found at: " + loc.getStartLine() + ":" + loc.getStartColumn()