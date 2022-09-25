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


- key must be utf-8 encoded string  
- value could be anything  


[![asciicast](https://asciinema.org/a/7Hpsp61HsisGQapO0uYF75w1j.svg)](https://asciinema.org/a/7Hpsp61HsisGQapO0uYF75w1j)  


```bash
$ yr
yr 0.2.3
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
    clear    clear all the keys, empty the table
    get      get the value of the given key
    help     Print this message or the help of the given subcommand(s)
    ls       list the latest updated key-values
    qr       show the QRCode for the given key
    set      set the value of the given key

$ yr set hello "world 😊"
$ yr set "world 😊" hello
$ yr set bytes "`head -c 64 /dev/random`"
$ yr get bytes
�H�=�K
      �.�(�_    1_��q��R���p��*ԍ]DԪ�S[��xY�@���D6�,�>J��#�
$ yr qr bytes
$ yr ls
```


