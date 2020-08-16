# `is-root`

A simple library to detect whether you are root/admin or not

## Installation

Add `is-root = "0.1.0"` to `[dependencies]` section in your `Cargo.toml`

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
