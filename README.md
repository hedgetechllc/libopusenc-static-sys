# About

`libopusenc-sys` is an FFI Rust-binding to the reference Opus Encoder library `libopusenc`.

It uses `bindgen` to dynamically generate all Rust bindings and documentation, and will
always compile a **static** version of the underlying Opus Encoder library to ensure
cross-system capability without having to ensure that Opus is installed on the target system.

The library is fully `no_std` compatible and represents the minimal amount of overhead
or glue code needed to utilize `libopusenc` within a Rust project.

## Building

In order to use this crate, you will need ??? installed on your build computer. TODO

## Installation

To use, add the following to your `Cargo.toml` file:

```
[dependencies]
libopusenc-sys = "1.0"
```

## License

This library is licensed under the [MIT license](http://opensource.org/licenses/MIT).
