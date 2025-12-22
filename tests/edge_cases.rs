//! Edge cases tests (Category 4)
//!
//! These tests validate robustness and edge cases.

use cfgx::cfgx;

// Test 4.1: Empty block
#[test]
fn test_empty_block() {
    cfgx! {
        #[cfg(feature = "never_enabled")] {
            // No items
        }
    }

    // Test passes if it compiles
}

// Test 4.2: Very long item list
#[test]
#[allow(dead_code)]
fn test_very_long_item_list() {
    cfgx! {
        #[cfg(test)] {
            const V1: u32 = 1;
            const V2: u32 = 2;
            const V3: u32 = 3;
            const V4: u32 = 4;
            const V5: u32 = 5;
            const V6: u32 = 6;
            const V7: u32 = 7;
            const V8: u32 = 8;
            const V9: u32 = 9;
            const V10: u32 = 10;
            const V11: u32 = 11;
            const V12: u32 = 12;
            const V13: u32 = 13;
            const V14: u32 = 14;
            const V15: u32 = 15;
            const V16: u32 = 16;
            const V17: u32 = 17;
            const V18: u32 = 18;
            const V19: u32 = 19;
            const V20: u32 = 20;
            const V21: u32 = 21;
            const V22: u32 = 22;
            const V23: u32 = 23;
            const V24: u32 = 24;
            const V25: u32 = 25;
        }
    }

    assert_eq!(V1, 1);
    assert_eq!(V10, 10);
    assert_eq!(V25, 25);
}

// Test 4.3: Nested cfg attributes in items
#[test]
fn test_nested_cfg_attributes() {
    cfgx! {
        #[cfg(test)] {
            #[cfg(unix)]
            const NESTED_UNIX: &str = "test-unix";

            #[cfg(windows)]
            const NESTED_WINDOWS: &str = "test-windows";

            #[cfg(not(any(unix, windows)))]
            const NESTED_OTHER: &str = "test-other";
        }
    }

    // One of these should exist based on platform
    #[cfg(all(test, unix))]
    assert_eq!(NESTED_UNIX, "test-unix");

    #[cfg(all(test, windows))]
    assert_eq!(NESTED_WINDOWS, "test-windows");
}

// Test 4.4: Items with other attributes
#[test]
fn test_items_with_other_attributes() {
    cfgx! {
        #[cfg(test)] {
            #[derive(Debug, Clone)]
            struct TestStruct {
                value: u32,
            }

            #[inline]
            fn test_fn() -> u32 { 42 }

            #[must_use]
            fn another_fn() -> u32 { 100 }

            #[allow(dead_code)]
            const MAYBE_UNUSED: u32 = 200;
        }
    }

    let s = TestStruct { value: 10 };
    let s2 = s.clone();
    assert_eq!(s2.value, 10);

    assert_eq!(test_fn(), 42);
    assert_eq!(another_fn(), 100);
}

// Test 4.5: Documentation comments
#[test]
fn test_documentation_comments() {
    cfgx! {
        #[cfg(test)] {
            /// This is a test constant
            /// With multiple lines
            /// of documentation
            const DOCUMENTED: u32 = 100;

            /// A documented function
            fn documented_fn() -> u32 {
                DOCUMENTED
            }

            /// A documented struct
            struct DocumentedStruct {
                /// A documented field
                value: u32,
            }
        }
    }

    assert_eq!(DOCUMENTED, 100);
    assert_eq!(documented_fn(), 100);
    let s = DocumentedStruct { value: 42 };
    assert_eq!(s.value, 42);
}

// Test 4.6: Visibility modifiers
#[test]
fn test_visibility_modifiers() {
    mod inner {
        use cfgx::cfgx;

        cfgx! {
            #[cfg(test)] {
                pub const PUBLIC: u32 = 1;
                pub(crate) const CRATE_PUBLIC: u32 = 2;
                pub(super) const SUPER_PUBLIC: u32 = 3;
                const PRIVATE: u32 = 4;
            }
        }

        pub fn test_private() -> u32 {
            PRIVATE
        }
    }

    assert_eq!(inner::PUBLIC, 1);
    assert_eq!(inner::CRATE_PUBLIC, 2);
    assert_eq!(inner::SUPER_PUBLIC, 3);
    assert_eq!(inner::test_private(), 4);
}

// Test 4.7: Generic items
#[test]
fn test_generic_items() {
    cfgx! {
        #[cfg(test)] {
            fn generic_fn<T>(value: T) -> T {
                value
            }

            fn generic_fn_with_bounds<T: Clone>(value: T) -> T {
                value.clone()
            }

            struct GenericStruct<T> {
                data: T,
            }

            struct GenericStructWithBounds<T: Clone> {
                data: T,
            }
        }
    }

    assert_eq!(generic_fn(42), 42);
    assert_eq!(generic_fn("test"), "test");

    assert_eq!(generic_fn_with_bounds(42), 42);

    let s1 = GenericStruct { data: 100 };
    assert_eq!(s1.data, 100);

    let s2 = GenericStructWithBounds { data: "test" };
    assert_eq!(s2.data, "test");
}

// Test 4.8: Lifetime parameters
#[test]
fn test_lifetime_parameters() {
    cfgx! {
        #[cfg(test)] {
            fn with_lifetime(s: &str) -> &str {
                s
            }

            fn with_multiple_lifetimes<'a>(s1: &'a str, _s2: &str) -> &'a str {
                s1
            }

            struct RefHolder<'a> {
                data: &'a str,
            }

            struct MultiRefHolder<'a, 'b> {
                data1: &'a str,
                data2: &'b str,
            }
        }
    }

    let text = "hello";
    assert_eq!(with_lifetime(text), "hello");

    let text2 = "world";
    assert_eq!(with_multiple_lifetimes(text, text2), "hello");

    let holder = RefHolder { data: "test" };
    assert_eq!(holder.data, "test");

    let multi = MultiRefHolder {
        data1: "first",
        data2: "second",
    };
    assert_eq!(multi.data1, "first");
    assert_eq!(multi.data2, "second");
}

// Additional test: Mixed generics and lifetimes
#[test]
fn test_mixed_generics_and_lifetimes() {
    cfgx! {
        #[cfg(test)] {
            struct Container<'a, T> {
                reference: &'a T,
                owned: T,
            }

            fn process<'a, T: Clone>(container: &Container<'a, T>) -> T {
                container.owned.clone()
            }
        }
    }

    let value = 42;
    let container = Container {
        reference: &value,
        owned: 100,
    };

    assert_eq!(*container.reference, 42);
    assert_eq!(container.owned, 100);
    assert_eq!(process(&container), 100);
}

// Additional test: Where clauses
#[test]
fn test_where_clauses() {
    cfgx! {
        #[cfg(test)] {
            fn with_where<T>(value: T) -> T
            where
                T: Clone + std::fmt::Debug,
            {
                value.clone()
            }

            struct WithWhere<T>
            where
                T: Clone,
            {
                data: T,
            }
        }
    }

    assert_eq!(with_where(42), 42);

    let s = WithWhere { data: "test" };
    assert_eq!(s.data, "test");
}

// Additional test: Async functions
#[test]
fn test_async_functions() {
    cfgx! {
        #[cfg(test)] {
            async fn async_fn() -> u32 {
                42
            }

            async fn async_with_param(x: u32) -> u32 {
                x * 2
            }
        }
    }

    // Just test that they compile
    // Actually running them would require an async runtime
    let _ = async_fn();
    let _ = async_with_param(10);
}

// Additional test: Unsafe items
#[test]
#[allow(dead_code)]
fn test_unsafe_items() {
    cfgx! {
        #[cfg(test)] {
            unsafe fn unsafe_fn() -> u32 {
                42
            }

            struct UnsafeWrapper {
                ptr: *const u32,
            }
        }
    }

    unsafe {
        assert_eq!(unsafe_fn(), 42);
    }

    let value = 100u32;
    let _wrapper = UnsafeWrapper {
        ptr: &value as *const u32,
    };
}
