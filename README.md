# About this
This is a fork from [gitlab](https://gitlab.com/caralice/is-root)

# `is-root`

![Downloads](https://img.shields.io/crates/d/is-root)
![License](https://img.shields.io/crates/l/is-root)
[![crates.io](https://img.shields.io/crates/v/is-root?logo=rust)](https://crates.io/crates/is-root)
[![docs.rs](https://docs.rs/is-root/badge.svg)](https://docs.rs/is-root)
[![Gitlab CI](https://img.shields.io/gitlab/pipeline/johnmeow/is-root/master?logo=gitlab)](https://gitlab.com/johnmeow/is-root/-/pipelines/latest)

A simple library to detect whether you are root/admin or not

## Installation

Add `is-root = "0.1.2"` to `[dependencies]` section in your `Cargo.toml`

## Usage

```rs
use is_root::is_root;

if is_root() {
    println!("Doing something dangerous")
} else {
    eprintln!("Run me as root")
}
```

You can find examples in [`examples`](/-/tree/master/examples) directory
