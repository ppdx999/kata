# kata
Declarative type checking commands

# Usage

```shell
kata <schema> <input>
```

# Example

### Valid input case

```terminal
$ cat data.txt
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh

$ cat data.txt | kata "id:integer email:string name:string"
00001 john@example.com   John_Doe
00002 sherry@example.com Sherry_Berry
00003 ram@example.com    Ram_Singh

$ echo $?
0 # this means `kata` command was successful
```


### Invalid input case

```terminal
$ cat data.txt
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh


$ cat data.txt | kata "id:integer email:string name:string"
00001 john@example.com John_Doe
00002                  Sherry_Berry
00003 ram@example.com  Ram_Singh

$ echo $?
1 # this means `kata` command was unsuccessful
```

kata never modify the input, it just checks the input against the schema and returns 0 if the input is valid and 1 if the input is invalid.
