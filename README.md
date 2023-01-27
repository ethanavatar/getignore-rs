# getignore-rs

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

### From Source

```bash
$ git clone https://github.com/ethanavatar/getignore-rs.git
$ cd getignore
$ cargo build --release
```

The binary is available at `target/release/getignore.exe`
