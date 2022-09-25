# youran
My personal adventures with rust!!!  

code with peace  



# Installation
```bash
cargo install youran
```

## Install `yr` only
```bash
cargo install youran --bin yr
```

# yr: a super stupid simple personal key-value store
It behaves like [kvass](https://github.com/maxmunzel/kvass)(which is written in go), but `without a server side`(maybe temporally)  

```bash
$ yr
yr 0.2.1
hangj <guijie.han@gmail.com>
code with peace

USAGE:
    yr [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --db <DB>    specify database file, use ~/.config/youran/db.sqlite by default if not set
                     environment variable YOURAN_DB_FILE
    -h, --help       Print help information
    -n, --newline    Do not print the trailing newline character
    -v, --verbose    give more verbose output
    -V, --version    Print version information

SUBCOMMANDS:
    clear    clear all the keys
    get      get the value of the given key
    help     Print this message or the help of the given subcommand(s)
    ls       list all the keys
    qr       show the QRCode for the given key
    set      set the value of the given key

$ yr set hello world
$ yr set world hello
$ yr ls
```
