# Library test

A test for making a library in rust

:warning: **this is a working in progress**

## How to use

1. Create a rust project

```bash
cargo new <my_project_name>
```

2. Add the library to cargo dependencies
```toml
[package]
name = "<my_project_name>"
version = "0.1.0"
edition = "2021"

[dependencies]
library_test = { git = "https://github.com/Caresle/library_test" }
```
