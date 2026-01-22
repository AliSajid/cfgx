// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Basic block syntax tests (Category 1)
//!
//! These tests validate the fundamental block syntax without if/else.
//! All tests use feature flags instead of platform-specific cfg to allow
//! comprehensive testing on any platform.

use cfgx::cfgx;

// Test 1.1: Single block with single item
#[test]
fn test_single_block_single_item() {
    cfgx! {
        #[cfg(test)] {
            const VALUE: u32 = 42;
        }
    }

    assert_eq!(VALUE, 42);
}

// Test 1.2: Single block with multiple items
#[test]
fn test_single_block_multiple_items() {
    cfgx! {
        #[cfg(not(feature = "never_enabled"))] {
            const A: &str = "alpha";
            const B: &str = "beta";
            const C: u32 = 100;
        }
    }

    assert_eq!(A, "alpha");
    assert_eq!(B, "beta");
    assert_eq!(C, 100);
}

// Test 1.3: Multiple separate blocks
#[test]
fn test_multiple_separate_blocks() {
    cfgx! {
        #[cfg(feature = "platform_a")] {
            const PLATFORM: &str = "a";
        }
        #[cfg(feature = "platform_b")] {
            const PLATFORM: &str = "b";
        }
        #[cfg(not(any(feature = "platform_a", feature = "platform_b")))] {
            const PLATFORM: &str = "default";
        }
    }

    // PLATFORM should exist and have one of the three values
    #[cfg(feature = "platform_a")]
    assert_eq!(PLATFORM, "a");

    #[cfg(feature = "platform_b")]
    assert_eq!(PLATFORM, "b");

    #[cfg(not(any(feature = "platform_a", feature = "platform_b")))]
    assert_eq!(PLATFORM, "default");
}

// Test 1.4: Non-cfg attributes
#[test]
fn test_non_cfg_attributes() {
    cfgx! {
        #[allow(dead_code)] {
            fn unused_function() -> u32 { 42 }
            const UNUSED: u32 = 0;
        }
    }

    // This test passes if it compiles without warnings
    // The items are marked as allowed to be dead code
}

// Test 1.5: Complex cfg conditions
#[test]
fn test_complex_cfg_conditions() {
    cfgx! {
        #[cfg(all(test, not(feature = "disable_ptr_size")))] {
            const PTR_SIZE: usize = 8;
        }
        #[cfg(all(test, feature = "disable_ptr_size"))] {
            const PTR_SIZE: usize = 4;
        }
    }

    // On normal test runs without the feature
    #[cfg(not(feature = "disable_ptr_size"))]
    assert_eq!(PTR_SIZE, 8);

    // With the feature enabled
    #[cfg(feature = "disable_ptr_size")]
    assert_eq!(PTR_SIZE, 4);
}

// Test 1.5b: Complex cfg with any
#[test]
#[allow(clippy::assertions_on_constants)]
fn test_complex_cfg_any() {
    cfgx! {
        #[cfg(any(feature = "opt1", feature = "opt2"))] {
            const IS_OPTION_ENABLED: bool = true;
        }
        #[cfg(not(any(feature = "opt1", feature = "opt2")))] {
            const IS_OPTION_ENABLED: bool = false;
        }
    }

    #[cfg(any(feature = "opt1", feature = "opt2"))]
    assert!(IS_OPTION_ENABLED);

    #[cfg(not(any(feature = "opt1", feature = "opt2")))]
    assert!(!IS_OPTION_ENABLED);
}

// Additional test: Empty block (edge case preview)
#[test]
fn test_empty_block() {
    cfgx! {
        #[cfg(feature = "never_enabled")] {
            // No items - this should compile fine
        }
    }

    // Test passes if it compiles
}

// Additional test: Block with multiple attributes on items
#[test]
fn test_items_with_multiple_attributes() {
    cfgx! {
        #[cfg(test)] {
            #[derive(Debug, Clone, Copy)]
            struct TestStruct {
                value: u32,
            }

            #[inline]
            #[must_use]
            const fn get_value() -> u32 {
                100
            }
        }
    }

    let s = TestStruct { value: 42 };
    let s2 = s; // Tests Clone/Copy
    assert_eq!(s.value, s2.value);
    assert_eq!(get_value(), 100);
}
