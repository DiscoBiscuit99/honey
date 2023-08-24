Done parsing:
- literal
- declaration
- statement

Diagram: lexer -> parser -> (type checker) -> code generation -> runtime

```bnf
<program> ::= <statement_list>

<statement_list> ::= <statement> ";" | <statement> ";" <statement_list>

<statement> ::= <declaration>

<declaration> ::= "let" <identifier> ":" <type> "=" <expression>

<type> ::= "int" | "string" | "bool" | "fn(" <type_list> ") =>" <type>

<type_list> ::= <type> | <type> "," <type_list>

<expression> ::= <literal> | <function_call> | <block>

<function_call> ::= <identifier> "(" <expression_list> ")"

<expression_list> ::= <expression> | <expression> "," <expression_list>

<literal> ::= <number> | <string>

<block> ::= "{" <statement_list> "}"
```
