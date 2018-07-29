# os_log

A Rust [`log`](https://github.com/rust-lang-nursery/log)
implementation that logs messages using Apple's new [unified logging
system](https://developer.apple.com/documentation/os/logging?language=occ).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
os_log = { git = "https://github.com/econobox/os_log" }
```

and this to your crate root:

```rust
extern crate os_log;
```

...and then don't do anything, because the library is a work in
progress and doesn't work yet :)

## License

`os_log` is licensed under the [MIT
License](https://opensource.org/licenses/MIT).
