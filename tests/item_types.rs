//! Different item types tests (Category 3)
//!
//! These tests validate that cfgx works with all Rust item types.

use cfgx::cfgx;

// Test 3.1: Functions
#[test]
fn test_functions() {
    cfgx! {
        #[cfg(test)] {
            fn test_helper() -> u32 { 42 }
            fn another_helper(x: u32) -> u32 { x * 2 }
        }
    }

    assert_eq!(test_helper(), 42);
    assert_eq!(another_helper(5), 10);
}

// Test 3.2: Structs and enums
#[test]
fn test_structs_and_enums() {
    cfgx! {
        if #[cfg(feature = "extended")] {
            struct ExtendedData {
                field_a: u32,
                field_b: String,
            }
            enum Mode { A, B, C }
        } else {
            struct ExtendedData {
                field_a: u32,
            }
            enum Mode { A, B }
        }
    }

    #[cfg(feature = "extended")]
    {
        let data = ExtendedData {
            field_a: 10,
            field_b: "test".to_string(),
        };
        assert_eq!(data.field_a, 10);

        let _mode = Mode::C;
    }

    #[cfg(not(feature = "extended"))]
    {
        let data = ExtendedData { field_a: 10 };
        assert_eq!(data.field_a, 10);

        let _mode = Mode::B;
    }
}

// Test 3.3: Impl blocks
#[test]
fn test_impl_blocks() {
    struct MyStruct {
        value: u32,
    }

    cfgx! {
        #[cfg(test)] {
            impl MyStruct {
                fn test_method(&self) -> bool {
                    self.value > 0
                }

                fn get_doubled(&self) -> u32 {
                    self.value * 2
                }
            }
        }
    }

    let s = MyStruct { value: 21 };
    assert!(s.test_method());
    assert_eq!(s.get_doubled(), 42);
}

// Test 3.4: Type aliases
#[test]
fn test_type_aliases() {
    cfgx! {
        if #[cfg(target_pointer_width = "64")] {
            type USize = u64;
            type ISize = i64;
        } else {
            type USize = u32;
            type ISize = i32;
        }
    }

    #[cfg(target_pointer_width = "64")]
    {
        let _x: USize = 42u64;
        let _y: ISize = -42i64;
    }

    #[cfg(target_pointer_width = "32")]
    {
        let _x: USize = 42u32;
        let _y: ISize = -42i32;
    }
}

// Test 3.5: Static values
#[test]
fn test_static_values() {
    cfgx! {
        #[cfg(test)] {
            static TEST_DATA: [u8; 4] = [1, 2, 3, 4];
            static COUNTER: std::sync::atomic::AtomicU32 =
                std::sync::atomic::AtomicU32::new(0);
        }
    }

    assert_eq!(TEST_DATA[0], 1);
    assert_eq!(TEST_DATA[3], 4);
    assert_eq!(COUNTER.load(std::sync::atomic::Ordering::Relaxed), 0);
}

// Test 3.6: Modules
#[test]
fn test_modules() {
    cfgx! {
        #[cfg(test)] {
            mod extra_module {
                pub fn helper() -> u32 { 42 }

                pub const VALUE: u32 = 100;
            }
        }
    }

    assert_eq!(extra_module::helper(), 42);
    assert_eq!(extra_module::VALUE, 100);
}

// Test 3.7: Use statements
#[test]
fn test_use_statements() {
    cfgx! {
        #[cfg(test)] {
            use std::collections::HashMap;
            use std::sync::Arc;
        }
    }

    let mut map = HashMap::new();
    map.insert("key", 42);
    assert_eq!(map.get("key"), Some(&42));

    let arc = Arc::new(100);
    assert_eq!(*arc, 100);
}

// Test 3.8: Trait definitions
#[test]
fn test_trait_definitions() {
    cfgx! {
        #[cfg(test)] {
            trait TestHelper {
                fn help(&self) -> u32;
            }

            trait AnotherHelper {
                fn process(&self, x: u32) -> u32;
            }
        }
    }

    struct MyType;

    impl TestHelper for MyType {
        fn help(&self) -> u32 {
            42
        }
    }

    impl AnotherHelper for MyType {
        fn process(&self, x: u32) -> u32 {
            x * 2
        }
    }

    let t = MyType;
    assert_eq!(t.help(), 42);
    assert_eq!(t.process(10), 20);
}

// Additional test: Const functions
#[test]
fn test_const_functions() {
    cfgx! {
        #[cfg(test)] {
            const fn compute_at_compile_time(x: u32) -> u32 {
                x * 2 + 10
            }

            const COMPUTED: u32 = compute_at_compile_time(16);
        }
    }

    assert_eq!(COMPUTED, 42);
    assert_eq!(compute_at_compile_time(5), 20);
}

// Additional test: Associated functions and types
#[test]
fn test_associated_items() {
    struct Container<T> {
        value: T,
    }

    cfgx! {
        #[cfg(test)] {
            impl<T> Container<T> {
                fn new(value: T) -> Self {
                    Container { value }
                }

                fn get(&self) -> &T {
                    &self.value
                }
            }
        }
    }

    let c = Container::new(42);
    assert_eq!(*c.get(), 42);
}

// Additional test: Macros (macro_rules)
#[test]
fn test_macro_definitions() {
    cfgx! {
        #[cfg(test)] {
            macro_rules! test_macro {
                ($x:expr) => {
                    $x * 2
                };
            }
        }
    }

    assert_eq!(test_macro!(21), 42);
}

// Additional test: Constants with complex types
#[test]
fn test_complex_const_types() {
    cfgx! {
        #[cfg(test)] {
            const ARRAY: [u32; 3] = [1, 2, 3];
            const TUPLE: (u32, &str, bool) = (42, "test", true);
            const NESTED: [[u8; 2]; 2] = [[1, 2], [3, 4]];
        }
    }

    assert_eq!(ARRAY[1], 2);
    assert_eq!(TUPLE.0, 42);
    assert_eq!(TUPLE.1, "test");
    assert_eq!(NESTED[1][0], 3);
}
