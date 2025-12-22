#[test]
fn test_desired_compile_errors() {
    // These are edge cases where we'd want better error messages:

    // 1. Empty if block with non-empty else (confusing)
    // cfgx! {
    //     if #[cfg(never)] {
    //         // empty
    //     } else {
    //         const VALUE: u32 = 1;
    //     }
    // }
    // This works but is confusing - could warn

    // 2. Using if/else with non-boolean cfg
    // cfgx! {
    //     if #[derive(Debug)] {  // ERROR: not a cfg
    //         struct Foo;
    //     } else {
    //         struct Foo;
    //     }
    // }
    // This should error early

    assert!(true, "Documentation test");
}
