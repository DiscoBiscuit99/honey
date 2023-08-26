Done parsing:
- literal
- declaration
- statement
- statement list

Diagram: lexer -> parser -> (type checker) -> code generation -> runtime

Ideas for optimization:
- Parsing:
    - provide alternative procedures that doesn't wrap the output in a syntax tree structure.
        - (this way, no unnecessary wrapping and unwrapping is required.)

```bnf
<program> ::= <statement_list>

<statement_list> ::= <statement> ";" | <statement> ";" <statement_list>

<statement> ::= <declaration>

<declaration> ::= "let" <identifier> ":" <type> "=" <expression>

<type> ::= "int" | "string" | "bool" | "fn(" <type_list> ") =>" <type>

<type_list> ::= <type> | <type> "," <type_list>

<expression> ::= <term> | <expression> "+" <term> | <expression> "-" <term>

<term> ::= <factor> | <term> "*" <factor> | <term> "/" <factor>

<factor> ::= <literal> | <function_call> | <block> | "(" <expression> ")"

<function_call> ::= <identifier> "(" <expression_list> ")"

<expression_list> ::= <expression> | <expression> "," <expression_list>

<literal> ::= <number> | <string>

<block> ::= "{" <statement_list> "}"
```
