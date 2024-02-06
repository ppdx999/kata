# kata
Declarative type checking commands

# Usage

```shell
kata <schema> <input>
```

# Example

### Valid input case

```terminal
$ kata "id:integer email:string name:string" <<EOF
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh
EOF # ↑input ↓output
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh
$ echo $?
0 # this means `kata` command was successful
```


### Invalid input case

```terminal
$ kata "id:integer email:string name:string" <<EOF
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh
EOF # ↑input ↓output
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh
$ echo $?
1 # this means `kata` command was unsuccessful
```
