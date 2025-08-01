```
Notation:
,     Concat
|     Alternative
{}     1 or more
[]    optional
()    Grouping
??    Special Form

if_ident                = newline, tab ;

tab                     = "\t" ;
newline                 = "\n" ;

program                 = {function_definition|expression|statement|string|bool|number|comment} ;
function_definition     = "fn" , identifier , "(" , [arguments] , ")" , "{" , {statement} , "}" ;
arguments               = expression , { "," , expression } ;
statement               = (variable_define | function_return) ";", 
statement_list          = statement, {statement} ;
[comment] ;
variable_define         = "let" , identifier , "=" , expression | if_assign ;
function_return         = "return" , (function_call | expression | value) ;
return_if               = "{", return", boolean ";", "}" ;
return_ifnum            = "{", "return", digit, ";", "}" ;
function_call           = identifier , "(" , [arguments] , ")" ;
if_expression           = "if", condition, return_if, "else", return_if ;
if_multi_line           = "if", boolean, "{", if_ident, "return", boolean, ";", newline, "}", "else", "{", if_ident, "return", boolean ";", newline, "}" ;

if_else                 = "if", condition, return_ifnum, "else", "if", condition, return_ifnum, "else", "{", "return", digit, "}" ;

if_assign               = "if", boolean, return_if, "else", return_if ;
condition               = value, relational_operator, value ;
if_expression_boolean   = "true" | "false" ;
expression              = boolean | math_expression | function_call | 
relational_operator     = ">" | "<" | "==" ;
number | string | identifier ;
math_expression         = value , { ("+" | "-") , value } ;
value                   = number | identifier | boolean | string ;
number                  = {digit} ;
boolean                 = "true" | "false" ;
string                  = "\"" , {alnum | " "} , "\"" ;
identifier              = alpha , <alnum> ;
alpha                   = ?alphabetic or equivalent character?;
alnum                   = ?alphanumeric character?;
digit                   = 0..9;
whitespace              = space | tab | newline | carriage_return; 
comment                 = "//", ?any character?
Note: The grammar as written doesn't handle whitespace, although the examples include it. You should handle it accordingly.
```
