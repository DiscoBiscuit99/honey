# Honey, I'm home 🍯

A *very sophisticated*, compiled programming language made for household use alone. 🐝

**Features**: #todo

- [ ] Constant propagation
- [ ] Single static assignment
- [ ] Dead code elimination
- [ ] Interoperability with C

**Example code**:

```honey
# this is a comment...

# basic "variable" declarations
let a: number = 1;      # result: a <- 1
mut b: number = a + 1;  # result: b <- a + 1
b += a;                 # result: b <- b + a

# tuples
# components data types should probably be the same
let some_tuple: (number, number) = (1, 2);

# tagged tuples
let tagged_tuple: (x: number, y: number) = (3, 4);
println("{tagged_tuple.x}, {tagged_tuple.y}");

# basic (single-variate) function/procedure declarations
let double_me: (x: number) -> number = {
    x * 2
};

# multi-variate/single-output function/procedure declarations
let double_and_add_us: (x: number, y: number) -> number = {
    x * 2 + y * 2
};

# multi-variate/multi-output function/procedure declarations
let double_us: (x: number, y: number) -> (number, number) = {
    (x * 2, y * 2)
};

# coolest function ever
let unit: (x: number) -> number = x;

# maybe even this (?)
let double_me_and_add_pi: (x: number) -> number = {
  x * 2 + pi
}, where pi: number = 3.14;

# then this should probably also be allowed 🤔
let fancy_idea: number = 2 * pi, where pi: number = 3.14;

# closures, maybe (?)
capture t: (x: number) -> number = x * 2;

# simple struct/record
let pet: struct = .{
    name: string,
    kind: animal_kind,

    #! docstring like this?
    let new: (name: string, kind: animal_kind) -> pet {
        .{ name: name, kind: kind }
        # maybe .{ name, kind } if variable names are identical to field names
    };

    #! for self-referencial instances (maybe?)
    let kind: (self) -> animal_kind {
        self.kind
    }
};

# simple struct/record (alt)
let pet: struct = .{
    pub name: string?,       # optional value (denoted by '?')
    pub kind: animal_kind,
     
    #! docstring like this?
    let new: (name: string, kind: animal_kind) -> pet {
        .{ name: name, kind: kind }
        # maybe .{ name, kind } if variable names are identical to field names
    };
     
    #! for self-referencial instances (maybe?)
    let kind: (self) -> animal_kind {
        self.kind
    }
      
    let name: (self) -> string? {
        self.name
    }
    
    let format: (self) -> string {
        let alt_name: string = {
            let emotion: string = "😞"
            "No name " .. emotion
        };
         
        self.name ! alt_name .. ", " .. self.kind
    }
};
```

**Syntax**:

```bnf
<program> ::= <statement-list>

<statement-list> ::= <statement> | <statement> <statement-list>

<statement> ::= <declaration> ";"

<declaration> ::= <decl-keyword> <identifier> ":" <type> "=" <expression>

<decl-keyword> ::= "let" | "mut"

<type> ::= <basic-type> | <func-type>

<basic-type> ::= "number"

<param-list> ::= <identifier> ":" <type> | <identifier> ":" <type> "," <param-list>

<func-type> ::= "(" <param-list> ")" "->" <type>

<expression> ::= <term> | <expression> "+" <term> | <expression> "-"<term> | <block>

<term> ::= <factor> | <term> "*" <factor> | <term> "/" <factor>

<factor> ::= <number>

<return-value> ::= <expression>

<block> ::= "{" <statement-list> <return-value> "}"
```

**Code generation**:

Implemented with a register machine (as opposed to a stack machine).

