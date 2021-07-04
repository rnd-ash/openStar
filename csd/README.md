## CSD library

This library executes CSD Runtime code found in DAS.

CSD is a scripting language DAS uses which can be used for certain procedures and functions with various ECUs.


### Keywords

* `ASSIGN` - Assigns a value to a variable
* `START` - Start of a function
* `ENDE` - End of a function
* `EXPR` - End of an expression
* `IF` - Start of if/else block
* `IFELSE` - Keyword for if/else block
* `IFONLY` - End of If block without else
* `END` - End of if/else block
* `GOTO` - Goto code on a specific line number
* `$GlobalVariables` - Special function at the top of every script. Assigns variables which will be used in the whole script.
* `$stack` - Assign to stack??
* `Array_Set` - ?? (Syntax unclear)
* `Array_Get` - ?? (Syntax unclear)
* `Array_Delete` - ?? (Syntax unclear)

### Variable types
* `INTEGER (I:)` - 32bit signed integer
* `FLOAT (F:)` - 32bit float
* `STRING (S:)` - ASCII String (null terminated)
* `(*:)` - ?? (Any data type??)



