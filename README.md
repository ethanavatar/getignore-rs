# getignore-rs

A CLI tool to fetch `.gitignore` file templates from [github/gitignore](https://github.com/github/gitignore).

## Usage

Create a new `.gitignore` file in the current directory

```bash
$ getignore Python
```

Overwrite the existing `.gitignore` file

```bash
$ getignore Python --force
```

## Installation

After installing Rust via [rustup](https://rustup.rs/). You can install using cargo:
```bash
$ cargo install getignore --git https://github.com/ethanavatar/getignore-rs.git
```
