// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Compile-fail tests (Category 7)
//!
//! These tests verify that certain invalid uses of cfgx produce
//! appropriate compile errors.
//!
//! NOTE: These tests require the `trybuild` crate to run properly.
//! They are currently disabled but documented here for future implementation.

// To enable these tests:
// 1. Add to Cargo.toml: trybuild = "1.0" in [dev-dependencies]
// 2. Create a tests/compile_fail/ directory
// 3. Put individual .rs files there (one per test case)
// 4. Run with: cargo test --test compile_fail

#[cfg(feature = "compile_fail_tests")]
mod compile_fail_tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();

        // Test 7.1: Using non-cfg with if/else
        t.compile_fail("tests/compile_fail/non_cfg_if_else.rs");

        // Test 7.2: Mismatched types in branches
        // Note: This is a Rust type error, not a macro error
        // The macro expands correctly, but rustc catches the type mismatch
        t.compile_fail("tests/compile_fail/mismatched_types.rs");

        // Test 7.3: Invalid cfg syntax
        t.compile_fail("tests/compile_fail/invalid_cfg_syntax.rs");
    }
}

// Workaround tests: Things that should fail but have workarounds
#[test]
#[allow(dead_code)]
fn test_workaround_patterns() {
    use cfgx::cfgx;

    // Pattern 1: Type mismatch - use generic code
    cfgx! {
        if #[cfg(feature = "high_precision")] {
            type Float = f64;
        } else {
            type Float = f32;
        }
    }

    // To use Float safely, write generic code:
    fn process<F: num_traits::Float>(value: F) -> F {
        value * F::from(2.0).unwrap()
    }

    // Or use type assertions at use sites:
    #[cfg(feature = "high_precision")]
    let _x: f64 = 1.0;

    #[cfg(not(feature = "high_precision"))]
    let _x: f32 = 1.0;

    assert!(true, "Workaround test");
}

// Note: To actually implement compile-fail tests, create files like:
//
// tests/compile_fail/non_cfg_if_else.rs:
// ```
// use cfgx::cfgx;
//
// fn main() {
//     cfgx! {
//         if #[allow(dead_code)] {
//             const VALUE: u32 = 1;
//         } else {
//             const VALUE: u32 = 2;
//         }
//     }
// }
// ```
//
// Then run: cargo test --test compile_fail
// trybuild will verify these produce the expected compile errors
