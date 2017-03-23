# hex2ascii
Command line app that converts hex values to/from ASCII.

### Help

```bash
$ hex2ascii -h
hex2ascii 0.2.1
Gavyn Riebau
Converts hex values to ascii
USAGE:
    hex2ascii [FLAGS]
FLAGS:
    -h, --help       Prints help information
    -r, --reverse    Converts from ascii to hex rather than the other way around
    -V, --version    Prints version information
    -v, --verbose    Include verbose output including warning messages written to stderr
```
### Encoding a message into hexadecimal

```bash
$ echo "Hello world" | hex2ascii -r
48656c6c6f20776f726c64
```

### Decoding a message from hexadecimal

```bash
$ echo "48656c6c6f20776f726c64" | hex2ascii
Hello world
```

