# schematch
Declarative type checking commands

# Usage

```txt
schematch <SCHEMA> [<FILE>]

SCHEMA:
    A schema is a string that describes the type of each field in the input.
    The schema is a space separated list of field types. Each field type is
    a string of the form <name>:<type> where <name> is the name of the field
    and <type> is the type of the field. The type can be one of the following:
    - integer
    - string
    - float
    - boolean
    - null
    
    Example:
    "id:integer email:string name:string"

FILE:
   if you omit this field, schematch read from stdin
```

# Example

### Valid input case

```terminal
$ cat data.txt
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh

$ cat data.txt | schematch "id:integer email:string name:string"
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh

$ echo $?
0 # this means `schematch` command was successful
```


### Invalid input case

```terminal
$ cat data.txt
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh


$ cat data.txt | schematch "id:integer email:string name:string"
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh

$ echo $?
1 # this means `schematch` command was unsuccessful
```

schematch never modify the input, it just checks the input against the schema and returns 0 if the input is valid and 1 if the input is invalid.


## Schema EBNF

```ebnf
schema ::= term ( ' ' term )*
term ::= name ':' type
name ::= [a-zA-Z_][a-zA-Z0-9_]*
type ::= 'integer' | 'string' | 'float' | 'boolean' | 'null'
```
