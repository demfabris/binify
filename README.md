[<img alt="github" src="https://img.shields.io/badge/github-demfabris/binify-8da0cb?style=for-the-badge&labelColor=555555&logo=github">](https://github.com/demfabris/binify)
![Crates.io](https://img.shields.io/crates/v/binify?style=for-the-badge)

# binify

A tool to convert config file attributes to binaries.

Have you ever had the need to read values from a config file during shell scripting?

Now you can:

_config.json_
```json
{
  "some": {
    "value": "read me!"
  }
}
```

_yourscript.sh_
```
binify config.json
echo $(some.value)
```

Output:
`read me!`

## Installation

#### From [crates.io](https://crates.io/crates/binify)
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

## Disclaimer

Do not generate binaries from unknown files
