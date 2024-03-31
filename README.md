> [!WARNING]
> This project is working on progress.

# schematch
Declarative schema checking commands

## Usage

```txt
Usage: schematch [OPTIONS] <SCHEMA> [FILE]

Arguments:
  <SCHEMA>  The schema to check against
  [FILE]    The file to check. If not provided, stdin will be used

Options:
  -s, --schema-type <SCHEMA_TYPE>  Schema type. schematch support tsv and json, If not provided tsv will be used [default: tsv] [possible values: tsv, json]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Quickstart

You can validate the schema of your data

for example when you have `data.txt`

```txt
1 john@example.com   John_Doe
2 sherry@example.com Sherry_Berry
3 ram@example.com    Ram_Singh
```

then you can validate your data's schema like this

```
$ cat data.txt | schematch "id:integer email:string name:string"
1 john@example.com   John_Doe
2 sherry@example.com Sherry_Berry
3 ram@example.com    Ram_Singh

$ echo $?
0
```


If your data is valid, then `schematch`'s exit code is 0, else 1.


Schematch don't modify recieved data and sends it to stdout as it is. Therefore, schematch can collaborate flexibly with other commands through 'pipe', and can be easily integrated into existing shell script pipeliens.

ex)

```terminal
$ cat data.txt                                    |
  schematch "id:integer email:string name:string" |
  awk '{print $1, $3}'                            |
  schematch "id:integer name:string"              |
  .
  .
  .
```

## Why?

Unix shell is beautifully small and highly composable. However, they are often less readable and understandable. This is partly because it is difficult to see how each command interacts with the data. To understand exactly what a shell script is doing, you need to understand the data structure of the files read by the `cat` and `find` commands, and how `awk` and `sed` process them.

However, shell scripts only describe the latter information (= how the data is processed). The former information (= how the original data and the processed data are structured) is always in the file and is never declaratively described in the shell script.

Schematch allows the former information(= data structure) to be included in the ShellScript pipeline.

For example, to use a shell to output the number of accesses per hour from an apache log file, the following shell script might be written

`cat /var/log/apache2/access.log | awk '{print $4}' | cut -b 2-15 | sort | uniq -c`

This certainly works well, but it is difficult to understand which command does what.

`Schematch` can change this to

```shell
cat /var/log/apache2/access.log                                              |
schematch 'ip:string localuser:string remoteuser:string time:string 
           res:string status:number bite:number referer:string agent:string" |
awk '{print $4}'                                                             |
tr ':/' ' '                                                                  |
schematch 'date:number month:string year:string h:number m:number s:number'  |
awk '{printf "%s_%s\n", $1, $2}'                                             |
schematch 'date_month:string'                                                |
sort                                                                         |
uniq -c
```

It describes more declaratively what the structure of the data is in the pipeline and how each command is processing the data semantically.

## Supported Schema

- tsv
- json

## Tsv

### Usage

```terminal
$ cat data.txt
1 jhon_doe@example.com  true  Jhon_Doe
2 emily_lua@example.com false Emily_Lua
3 mac_kily@example.com  true  _
```

```terminal
$ cat data.txt | schematch "id:integer  email:string  is_active:boolean  name:string|null" > /dev/null

$ echo $?
0
```

Supported type and value

| type | valid value | invalid value |
| -- | --| -- |
| integer |  `1`, `449`, `-4` , etc.. | `1.0`, `x123` |
| float   |  `1`, `4.0`, `-39`, etc.. | `xx`, `yy` |
| string  |  `aaa`, `bbb`, `c`, etc.. | String accpet anything |
| boolean |  `true`, `false` | all chars other than valid case is invalid |
| null    |  `_` | all chars other than valid case is invalid |


## Json

```terminal
$ cat data.txt
{
  "group": "Group1",
  "members": [
    {"id": "aaa", "name": "Jhon"},
    {"id": "bbb", "name": "Mary"}
  ]
}
```

```terminal
$ cat data.txt | schematch --schema-type json "{group: string, members: Array<{id: string, name: string}>}" > /dev/null

$ echo $?
0
```

Supported Types

- null
- string
- number
- boolean
- object
