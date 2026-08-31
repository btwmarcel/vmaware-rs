# vmaware-rs

Rust bindings to [VMAware](https://github.com/NotRequiem/VMAware), a cross-platform library for virtual machine detection.

## Requirements

- C++ compiler
- libclang

## Usage

```toml
[dependencies]
vmaware = { git = "https://github.com/btwmarcel/vmaware-rs" }
```

See the [basic](examples/basic.rs) example for usage.

## License

Licensed under the MIT License, matching upstream VMAware. The vendored VMAware header is at `deps/vmaware.hpp`.
