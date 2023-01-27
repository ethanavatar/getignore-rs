# getignore-rs &emsp; [![Latest Version]][crates.io]

[crates.io]: https://crates.io/crates/serde

A CLI tool to fetch `.gitignore` file templates

## Usage

Create a new `.gitignore` file in the current directory

```bash
$ ./getignore Python
```

Overwrite the existing `.gitignore` file

```bash
$ ./getignore Python --force
```

## Installation

### From Crates.io

```bash
$ cargo install getignore
```

### From Source

```bash
$ git clone https://github.com/ethanavatar/getignore-rs.git
$ cd getignore
$ cargo install --path .
```