/**
 * @name Database Types - Files
 * @description Detects file information in CodeQL database
 * @kind problem
 * @problem.severity warning
 * @id rust/database-types/files
 */

import rust

// Look for file information
from File file
select file, "Source file found: " + file.getBaseName()