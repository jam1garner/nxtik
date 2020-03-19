# nxtik

A library and tool for parsing and outputting information about Switch .tik files

```
nxtik 1.0.0

USAGE:
    nxtik [FLAGS] <file>

FLAGS:
    -h, --help         Prints help information
    -k, --key-only     
    -r, --rid-only     
    -s, --signature    
    -V, --version      Prints version information

ARGS:
    <file>
```

## Installing

Proper install (requires Rust to be installed, supports all Operating Systems):

```
cargo install --git https://github.com/jam1garner/nxtik
```

Rust-less install (Windows and Linux only):

1. Download from [the releases page](https://github.com/jam1garner/nxtik/releases) (no MacOS build available)
2. Copy the executable (`nxtik` for Linux or `nxtik.exe` for Windows) to either a folder added to path or to wherever you want to use it

## Build from source

```
   git clone https://github.com/jam1garner/nxtik
   cd nxtik
   cargo build --release
```
