//! If/Else syntax tests (Category 2)
//!
//! These tests validate the if/else syntax for mutually exclusive branches.

use cfgx::cfgx;

// Test 2.1: Simple if/else with single item
#[test]
fn test_simple_if_else_single_item() {
    cfgx! {
        if #[cfg(test)] {
            const MODE: &str = "testing";
        } else {
            const MODE: &str = "production";
        }
    }

    // In test builds, MODE should be "testing"
    assert_eq!(MODE, "testing");
}

// Test 2.2: If/else with multiple items
#[test]
fn test_if_else_multiple_items() {
    cfgx! {
        if #[cfg(target_pointer_width = "64")] {
            const BITS: u8 = 64;
            type PointerInt = i64;
        } else {
            const BITS: u8 = 32;
            type PointerInt = i32;
        }
    }

    // On 64-bit systems
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(BITS, 64);
        let _x: PointerInt = 42i64;
    }

    // On 32-bit systems
    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(BITS, 32);
        let _x: PointerInt = 42i32;
    }
}

// Test 2.3: Multiple if/else blocks in one invocation
#[test]
fn test_multiple_if_else_blocks() {
    cfgx! {
        if #[cfg(unix)] {
            const OS_TYPE: &str = "unix";
        } else {
            const OS_TYPE: &str = "other";
        }

        if #[cfg(target_pointer_width = "64")] {
            const ARCH_BITS: u8 = 64;
        } else {
            const ARCH_BITS: u8 = 32;
        }
    }

    // Both constants should exist
    #[cfg(unix)]
    assert_eq!(OS_TYPE, "unix");

    #[cfg(not(unix))]
    assert_eq!(OS_TYPE, "other");

    #[cfg(target_pointer_width = "64")]
    assert_eq!(ARCH_BITS, 64);

    #[cfg(target_pointer_width = "32")]
    assert_eq!(ARCH_BITS, 32);
}

// Test 2.4: Empty else block
#[test]
fn test_empty_else_block() {
    cfgx! {
        if #[cfg(feature = "experimental")] {
            const EXPERIMENTAL: bool = true;
        } else {
            // Empty - no items
        }
    }

    // If feature is not enabled, EXPERIMENTAL won't exist
    // This test passes if it compiles
    #[cfg(feature = "experimental")]
    assert_eq!(EXPERIMENTAL, true);
}

// Test 2.5: Complex types in branches
#[test]
fn test_complex_types_in_branches() {
    cfgx! {
        if #[cfg(feature = "high_precision")] {
            type Float = f64;
            const EPSILON: f64 = 1e-10;
            fn precision_name() -> &'static str { "high" }
        } else {
            type Float = f32;
            const EPSILON: f32 = 1e-6;
            fn precision_name() -> &'static str { "standard" }
        }
    }

    // All three items should exist with consistent types
    #[cfg(feature = "high_precision")]
    {
        let _x: Float = 1.0f64;
        assert_eq!(EPSILON, 1e-10);
        assert_eq!(precision_name(), "high");
    }

    #[cfg(not(feature = "high_precision"))]
    {
        let _x: Float = 1.0f32;
        assert_eq!(EPSILON, 1e-6);
        assert_eq!(precision_name(), "standard");
    }
}

// Additional test: if/else with structs that have different shapes
#[test]
fn test_different_struct_shapes() {
    cfgx! {
        if #[cfg(feature = "extended")] {
            struct Config {
                name: String,
                value: u32,
                extra: bool,
            }

            const DEFAULT_EXTRA: bool = true;
        } else {
            struct Config {
                name: String,
                value: u32,
            }

            const DEFAULT_EXTRA: bool = false;
        }
    }

    // Test that the appropriate struct exists
    #[cfg(feature = "extended")]
    {
        let config = Config {
            name: "test".to_string(),
            value: 42,
            extra: true,
        };
        assert_eq!(config.value, 42);
        assert_eq!(DEFAULT_EXTRA, true);
    }

    #[cfg(not(feature = "extended"))]
    {
        let config = Config {
            name: "test".to_string(),
            value: 42,
        };
        assert_eq!(config.value, 42);
        assert_eq!(DEFAULT_EXTRA, false);
    }
}

// Additional test: if/else with functions that have different implementations
#[test]
fn test_different_function_implementations() {
    cfgx! {
        if #[cfg(debug_assertions)] {
            fn compute(x: u32) -> u32 {
                // Debug version with bounds checking
                x.checked_add(10).unwrap()
            }
            const IS_DEBUG: bool = true;
        } else {
            fn compute(x: u32) -> u32 {
                // Release version, simple add
                x + 10
            }
            const IS_DEBUG: bool = false;
        }
    }

    assert_eq!(compute(5), 15);

    #[cfg(debug_assertions)]
    assert!(IS_DEBUG);

    #[cfg(not(debug_assertions))]
    assert!(!IS_DEBUG);
}
