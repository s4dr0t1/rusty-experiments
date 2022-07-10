## Introduction

A barely functional reconnaissance framework written in Rust. A WIP at the moment, expect everything to be broken (yet).

```
hunter 
s4dr0t1

USAGE:
    hunter <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    enumerate    Subdomain enumeration using crt.sh API
    help         Print this message or the help of the given subcommand(s)

```

## Building

No GitHub releases for now, project is in too early stages.


```sh
sudo apt install pkg-config libssl-dev curl -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable
cargo build --release
```
