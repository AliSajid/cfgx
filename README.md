<!--
SPDX-FileCopyrightText: 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# `cfgx` - Extended Conditional Compilation

A Rust macro library for applying conditional compilation attributes to blocks
of items, with support for if/else syntax.

## Features

- **Block Syntax**: Apply any attribute to multiple items at once
- **If/Else Syntax**: Mutually exclusive branches with `cfg` attributes
- **Zero Overhead**: Pure compile-time macro expansion
- **Comprehensive**: Works with all Rust item types (`const`, `fn`, `struct`, `enum`, `impl`, etc.)
- **Type Safe**: Full Rust type checking applies

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cfgx = "0.1.0"
```

## Quick Start

```rust
use cfgx::cfgx;

cfgx! {
    if #[cfg(unix)] {
        const OS: &str = "unix";
        const PATH_SEP: char = '/';
    } else {
        const OS: &str = "windows";
        const PATH_SEP: char = '\\';
    }
}
```

## Syntax

### Block Syntax

Apply any attribute to a group of items:

```rust
cfgx! {
    #[cfg(unix)] {
        const PLATFORM: &str = "unix";
        fn platform_init() { /* unix-specific */ }
    }

    #[cfg(windows)] {
        const PLATFORM: &str = "windows";
        fn platform_init() { /* windows-specific */ }
    }
}
```

### If/Else Syntax

For mutually exclusive branches (only works with `cfg` attributes):

```rust
cfgx! {
    if #[cfg(target_pointer_width = "64")] {
        type PointerInt = i64;
        const BITS: u8 = 64;
    } else {
        type PointerInt = i32;
        const BITS: u8 = 32;
    }
}
```

### Multiple Conditions

You can have multiple independent if/else blocks:

```rust
cfgx! {
    if #[cfg(unix)] {
        const OS_TYPE: &str = "unix";
    } else {
        const OS_TYPE: &str = "other";
    }

    if #[cfg(debug_assertions)] {
        const MODE: &str = "debug";
    } else {
        const MODE: &str = "release";
    }
}
```

## Use Cases

### Precision Selection

```rust
cfgx! {
    if #[cfg(feature = "f64_precision")] {
        type Score = f64;
        type Probability = f64;
        const MIN_SCORE: f64 = 1e-10;
    } else {
        type Score = f32;
        type Probability = f32;
        const MIN_SCORE: f32 = 1e-6;
    }
}
```

### Platform-Specific Code

```rust
cfgx! {
    if #[cfg(unix)] {
        const PATH_SEPARATOR: char = '/';
        fn normalize_path(path: &str) -> String {
            path.replace('\\', "/")
        }
    } else {
        const PATH_SEPARATOR: char = '\\';
        fn normalize_path(path: &str) -> String {
            path.replace('/', "\\")
        }
    }
}
```

### Algorithm Selection

```rust
cfgx! {
    if #[cfg(feature = "simd")] {
        fn dot_product(a: &[f32], b: &[f32]) -> f32 {
            // SIMD implementation
        }
        const ALGORITHM: &str = "simd";
    } else {
        fn dot_product(a: &[f32], b: &[f32]) -> f32 {
            // Scalar implementation
        }
        const ALGORITHM: &str = "scalar";
    }
}
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test category
cargo test --test basic_blocks
cargo test --test if_else
cargo test --test item_types
cargo test --test edge_cases
cargo test --test real_world

# Run with specific features
cargo test --features f64_precision
cargo test --features simd
```

### Test-Driven Development

This project follows TDD. All tests are written first, and the implementation
follows. Test categories:

1. **`basic_blocks.rs`** - Basic block syntax without if/else
2. **`if_else.rs`** - If/else syntax with `cfg` attributes
3. **`item_types.rs`** - All Rust item types (`fn`, `struct`, `enum`, etc.)
4. **`edge_cases.rs`** - Edge cases and robustness
5. **`real_world.rs`** - Practical use cases

### Current Implementation Status

- [X] Phase 1: Basic block syntax (Tests 1.1-1.5)
- [X] Phase 2: If/else syntax (Tests 2.1-2.5)
- [X] Phase 3: All item types (Tests 3.1-3.8)
- [X] Phase 4: Edge cases (Tests 4.1-4.8)
- [X] Phase 5: Real-world scenarios (Tests 8.1-8.7)

## Comparison with `cfg_block`

`cfgx` is inspired by the unmaintained `cfg_block` crate but aims to:

- Provide clearer naming ("`cfgx`" = "`cfg` extended")
- Support future enhancements (else-if chains, mixed syntax)
- Better documentation and examples
- Comprehensive test coverage
- Active maintenance

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Future Enhancements

Potential future features:

- [ ] `else if` chains
- [ ] Mix block and if/else syntax in one invocation
- [ ] Better error messages with `compile_error!`
- [ ] Support for expression-level conditionals
- [ ] Documentation improvements
