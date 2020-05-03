# `gib` - A `.gitignore` bootstrapper for projects using `git`
![Crates.io](https://img.shields.io/crates/v/gib)
![Travis CI build](https://travis-ci.com/DavSanchez/gib.svg?branch=master)
![License](https://img.shields.io/crates/l/gib/0.1.0)

This is a small utility for those who need to generate `.gitignore` files for different languages or frameworks. `gib` uses `.gitignore` templates, allowing to check and generate these files from them. The templates are collected directly from [GitHub's own gitignore repository](https://github.com/github/gitignore).

## Installation
You can download the binaries for the available platforms [at GitHub's releases page](https://github.com/DavSanchez/gib/releases)

### (macOS) Homebrew
```bash
brew install davsanchez/gib/gib
```

Or `brew tap davsanchez/gib` and then `brew install gib`.

### From [crates.io](https://crates.io/crates/gib)
If you have installed [Rust](https://www.rust-lang.org) on your machine, you can just do:
```bash
cargo install gib
```

### Installing from source
You'll also need [Rust](https://www.rust-lang.org) installed for this one:
```bash
git clone https://github.com/DavSanchez/gib.git --recurse-submodules
cd gib
cargo install
```

## Usage
### Create `.gitignore` at current directory (if it doesn't exist)
```bash
gib [<template>...] 
```

If a `.gitignore` file already exists at that location, `gib` will do nothing.

If you want to extend an existing file instead, use the `-a|--append` flag. If you want to overwrite it, use `-r|--replace` (append takes precedence over replace).

### Create `.gitignore` at other directory
```bash
gib [<template>...] [-o|--output] [<path>]
```

### Get list of available templates
```bash
gib [-l|--list]
```

### Print result of specified templates to `stdout` only
```bash
gib [<template>...] [-s|--show]
```

This flag takes precedence over `--append`, `--replace` and `--output` flags.

## Examples
### Output `.gitignore` file for Go and Rust
```bash
gib go rust
```
#### `.gitignore`
```
###############
#   Go
###############
# Binaries for programs and plugins
*.exe
*.exe~
*.dll
*.so
*.dylib

# Test binary, built with `go test -c`
*.test

# Output of the go coverage tool, specifically when used with LiteIDE
*.out

# Dependency directories (remove the comment below to include it)
# vendor/

###############
#   Rust
###############
# Generated by Cargo
# will have compiled files and executables
/target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk
```
## Pending changes
- [ ] Additional means of installation (`brew` for macOS, `scoop` or `choco` for Windows, etc.)
- [ ] Good manpages, completion and documentation.
- [ ] Replace `.gitignore` template loading with [`lazy_static`](https://docs.rs/lazy_static/) or [`phf`](https://github.com/sfackler/rust-phf).
- [ ] Code's a mess, to be honest. Must find a way to refactor and design it better, more idiomatic.