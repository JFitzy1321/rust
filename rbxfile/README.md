# rbxfile

Rust By Example File tool

A cli tool written in Rust that creates rs files in your current directory, with some added things that I always type in when following along with the "Rust By Example" Book.

## Installation

- You must have Rust stable installed
- Download or clone the Repo
- Run cargo build --release
- Move the executable to a folder in your PATH
  - For \*nix systems, run deploy.sh. This will move the exectuable to the bin folder in your \$HOME directory
  - For Windows, run ' cargo build --release ' and move executable to a folder in your %PATH%
    - or download git bash and use that as your terminal emulator

## Running the tool

Open cmd/terminal (whatever cli you use) and type:

```
rbxfile [some file name]
```

This will create a .rs file in your current directory
