# About

`libopusenc-static-sys` is an FFI Rust-binding to the reference Opus Encoder library `libopusenc`.

It uses `bindgen` to dynamically generate all Rust bindings and documentation, and will
always compile a **static** version of the underlying Opus Encoder library to ensure
cross-system capability without having to ensure that Opus is installed on the target system.

The library is fully `no_std` compatible and represents the minimal amount of overhead
or glue code needed to utilize `libopusenc` within a Rust project.

Rust documentation can be found [here](https://docs.rs/libopusenc-static-sys/latest), which
corresponds directly to the `libopusenc` [documentation](https://opus-codec.org/docs/libopusenc_api-0.2/).

## Building

In order to use this crate, you will need `cmake` installed on your build computer. Most
Linux-based operating systems provide this via the built-in package manager. It is available
on MacOS from `Homebrew`. On Windows, it should have been installed by default when you
installed the Visual Studio compiler.

## Installation

To use, add the following to your `Cargo.toml` file:

```
[dependencies]
libopusenc-static-sys = "1.0"
```

## License

This library is licensed under the [MIT license](http://opensource.org/licenses/MIT).
