#[test]
fn test_error_documentation() {
    // Test 7.1: Using non-cfg attribute with if/else
    //
    // This should ideally produce an error:
    // ```compile_fail
    // cfgx! {
    //     if #[allow(dead_code)] {  // ERROR: not a cfg attribute
    //         const VALUE: u32 = 1;
    //     } else {
    //         const VALUE: u32 = 2;
    //     }
    // }
    // ```
    //
    // Currently, this expands to:
    // #[allow(dead_code)] const VALUE: u32 = 1;
    // #[not(allow(dead_code))] const VALUE: u32 = 2;
    //
    // The second line is invalid cfg syntax, which rustc will catch.
    // Ideally, the macro would catch this earlier with a better error.

    assert!(true, "Documentation test");
}
