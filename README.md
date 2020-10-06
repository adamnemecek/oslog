# oslog

A Rust [`log`](https://github.com/rust-lang-nursery/log)
implementation that logs messages using Apple's new [unified logging
system](https://developer.apple.com/documentation/os/logging?language=occ).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
oslog = { git = "https://git.sr.ht/~nerosnm/oslog" }
```

and this to your crate root:

```rust
extern crate oslog;
```

...and then don't do anything, because the library is a work in
progress and doesn't work yet :)

## Resources
`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/os/trace_base.h`

## License

`oslog` is licensed under the [MIT
License](https://opensource.org/licenses/MIT).
