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

```rust
use vmaware::{detect, brand, vm_type, conclusion, percentage, detected_count, check, flags};

fn main() -> Result<(), vmaware::VmawareError> {
    println!("is vm: {}", detect()?);
    println!("brand: {}", brand()?);
    println!("type: {}", vm_type()?);
    println!("conclusion: {}", conclusion()?);
    println!("percentage: {}%", percentage()?);
    println!("detected techniques: {}", detected_count()?);

    // check a single technique
    if check(flags::HYPERVISOR_BIT)? {
        println!("hypervisor bit is set");
    }

    Ok(())
}
```

## License

Licensed under the MIT License, matching upstream VMAware. The vendored VMAware header is at `deps/vmaware.hpp`.
