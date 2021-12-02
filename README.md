# COMPACTO

[![ci](https://github.com/eduardostuart/compacto/actions/workflows/ci.yml/badge.svg)](https://github.com/eduardostuart/compacto/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/compacto/badge.svg)](https://docs.rs/compacto/)

> A fast way to compress & decompress JSON 

## Lib

To use this lib in your project, add the following line to your `Cargo.toml` file:

```toml
compacto = { version = "1.0.1" }
```

You can find the full documentation on [Docs.rs](https://docs.rs/compacto).

## Examples

There are a few examples in the `/examples` folder if you want to run it locally:

```bash
cargo run --release --example example-filename
```

## CLI

### Usage

Compress a JSON file/string:
```bash
compacto ./input-file.json ./output.compacto.json -c
```

Decompress a JSON file/string:
```bash
compacto ./compacto-file.compacto.json ./output.json -d
```

<!-- ### Installation

**From binaries**

Check out the [release page](#) for prebuilt versions of `COMPACTO`.

**From source**

If you want to build `COMPACTO` from source, you'll need Rust >= 1.56.1 or higher. 

```bash
cargo install --locked compacto
``` -->


## License

`COMPACTO` is made available under the terms of MIT License.

See the [LICENSE-MIT](./LICENSE) for license details.
