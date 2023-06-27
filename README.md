# binify

A tool to convert config file attributes to binaries.

## Installation

### from crates.io
`cargo install binify`

## Example

### JSON

_some.json_
```json
{
  "foo": "bar",
  "baz": {
    "quz": "qork"
  },
  "boo": [
    "bah",
    {
      "lol": "lurg"
    }
  ]
}
```

#### `$ binify some.json`

Generates:
```
$ ls
foo
baz.quz
boo.0
boo.1.lol
```

Outputs:
```
$ ./foo
bar

$ ./baz.quz
qork

$ ./boo.0
bah

$ ./boo.1.lol
lurg
```

### Env

_example.env_
```
FOO=BAR
BAZ=BORG
```

#### `$ binify example.env`

Generates:
```
$ ls
FOO
BAZ
```

Outputs:
```
$ ./FOO
BAR

$ ./BAZ
BORG
```
