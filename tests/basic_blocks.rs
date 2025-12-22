//! Basic block syntax tests (Category 1)
//!
//! These tests validate the fundamental block syntax without if/else.

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
        #[cfg(not(mips))] {
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
        #[cfg(unix)] {
            const PLATFORM: &str = "unix";
        }
        #[cfg(windows)] {
            const PLATFORM: &str = "windows";
        }
        #[cfg(target_family = "wasm")] {
            const PLATFORM: &str = "wasm";
        }
    }
    
    // PLATFORM should exist and have one of the three values
    // depending on the target platform
    #[cfg(unix)]
    assert_eq!(PLATFORM, "unix");
    
    #[cfg(windows)]
    assert_eq!(PLATFORM, "windows");
    
    #[cfg(target_family = "wasm")]
    assert_eq!(PLATFORM, "wasm");
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
        #[cfg(all(unix, target_pointer_width = "64"))] {
            const PTR_SIZE: usize = 8;
        }
        #[cfg(all(unix, target_pointer_width = "32"))] {
            const PTR_SIZE: usize = 4;
        }
        #[cfg(windows)] {
            const PTR_SIZE: usize = 8; // Assume 64-bit for test
        }
    }
    
    // On 64-bit Unix systems
    #[cfg(all(unix, target_pointer_width = "64"))]
    assert_eq!(PTR_SIZE, 8);
    
    // On 32-bit Unix systems
    #[cfg(all(unix, target_pointer_width = "32"))]
    assert_eq!(PTR_SIZE, 4);
    
    // On Windows
    #[cfg(windows)]
    assert_eq!(PTR_SIZE, 8);
}

// Test 1.5b: Complex cfg with any
#[test]
fn test_complex_cfg_any() {
    cfgx! {
        #[cfg(any(windows, target_family = "wasm"))] {
            const IS_WINDOWS_OR_WASM: bool = true;
        }
        #[cfg(not(any(windows, target_family = "wasm")))] {
            const IS_WINDOWS_OR_WASM: bool = false;
        }
    }
    
    #[cfg(any(windows, target_family = "wasm"))]
    assert_eq!(IS_WINDOWS_OR_WASM, true);
    
    #[cfg(not(any(windows, target_family = "wasm")))]
    assert_eq!(IS_WINDOWS_OR_WASM, false);
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
