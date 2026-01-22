// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![no_std]
//! # cfgx - Extended conditional compilation blocks
//!
//! `cfgx` provides a simple macro for applying conditional compilation
//! attributes to blocks of items, with support for if/else syntax.
//!
//! ## Quick Start
//!
//! ```
//! use cfgx::cfgx;
//!
//! cfgx! {
//!     if #[cfg(unix)] {
//!         const OS: &str = "unix";
//!         const PATH_SEP: char = '/';
//!     } else {
//!         const OS: &str = "windows";
//!         const PATH_SEP: char = '\\';
//!     }
//! }
//!
//! // Use the constants
//! assert_eq!(PATH_SEP, '/');  // On Unix systems
//! ```
//!
//! ## Features
//!
//! - Apply attributes to multiple items at once
//! - Support for if/else syntax with `cfg` attributes
//! - Works with any Rust item (const, fn, struct, enum, etc.)
//! - Zero runtime overhead - pure compile-time macro

/// Apply conditional compilation attributes to blocks of items.
///
/// # Syntax
///
/// There are two main syntax forms:
///
/// ## Block Syntax
///
/// Apply any attribute to a block of items:
///
/// ```
/// # use cfgx::cfgx;
/// cfgx! {
///     #[cfg(unix)] {
///         const PLATFORM: &str = "unix";
///         const VALUE: u32 = 1;
///     }
///     #[cfg(windows)] {
///         const PLATFORM: &str = "windows";
///         const VALUE: u32 = 2;
///     }
/// }
/// ```
///
/// ## If/Else Syntax
///
/// Use if/else for mutually exclusive branches (only works with `cfg`):
///
/// ```
/// # use cfgx::cfgx;
/// cfgx! {
///     if #[cfg(target_pointer_width = "64")] {
///         const BITS: u8 = 64;
///         type IntPtr = i64;
///     } else {
///         const BITS: u8 = 32;
///         type IntPtr = i32;
///     }
/// }
/// ```
///
/// # Examples
///
/// ## Multiple independent conditions
///
/// ```
/// # use cfgx::cfgx;
/// cfgx! {
///     if #[cfg(unix)] {
///         const OS_TYPE: &str = "unix";
///     } else {
///         const OS_TYPE: &str = "other";
///     }
///
///     if #[cfg(target_pointer_width = "64")] {
///         const ARCH_BITS: u8 = 64;
///     } else {
///         const ARCH_BITS: u8 = 32;
///     }
/// }
/// ```
///
/// ## Complex cfg conditions
///
/// ```
/// # use cfgx::cfgx;
/// cfgx! {
///     #[cfg(all(unix, target_pointer_width = "64"))] {
///         const PLATFORM_ID: &str = "unix-64";
///     }
///     #[cfg(any(windows, target_family = "wasm"))] {
///         type PlatformInt = i32;
///     }
/// }
/// ```
///
/// ## Different item types
///
/// ```
/// # use cfgx::cfgx;
/// cfgx! {
///     if #[cfg(feature = "extended")] {
///         struct Data {
///             field_a: u32,
///             field_b: String,
///         }
///
///         fn process(data: &Data) -> u32 {
///             data.field_a
///         }
///
///         const FEATURE_ENABLED: bool = true;
///     } else {
///         struct Data {
///             field_a: u32,
///         }
///
///         fn process(data: &Data) -> u32 {
///             data.field_a
///         }
///
///         const FEATURE_ENABLED: bool = false;
///     }
/// }
/// ```
#[macro_export]
macro_rules! cfgx {
    // Rule for if/else with cfg attributes
    // Pattern: if #[cfg(...)] { items } else { items }
    //
    // This rule handles zero or more if/else blocks.
    // For each block:
    //   - Items in the if branch get #[cfg($meta)]
    //   - Items in the else branch get #[cfg(not($meta))]
    (
        $(
            if #[cfg($meta:meta)] {
                $($item:item)*
            } else {
                $($item_f:item)*
            }
        )*
    ) => {
        $(
            // Expand the if branch: apply #[cfg($meta)] to each item
            $(
                #[cfg($meta)]
                $item
            )*
            // Expand the else branch: apply #[cfg(not($meta))] to each item
            $(
                #[cfg(not($meta))]
                $item_f
            )*
        )*
    };

    // Rule for blocks with any attribute
    // Pattern: #[attribute] { items }
    //
    // This rule handles zero or more attribute blocks.
    // For each block, every item gets the attribute applied.
    (
        $(
            #[$meta:meta] {
                $( $item:item )*
            }
        )*
    ) => {
        $(
            // For each block, apply the attribute to each item
            $(
                #[$meta]
                $item
            )*
        )*
    };
}

#[cfg(test)]
mod tests {
    // Basic smoke test
    #[test]
    fn macro_exists() {
        cfgx! {
            #[cfg(test)] {
                const TEST_VALUE: u32 = 42;
            }
        }
        assert_eq!(TEST_VALUE, 42);
    }

    // Test if/else syntax
    #[test]
    fn if_else_works() {
        cfgx! {
            if #[cfg(test)] {
                const MODE: &str = "testing";
            } else {
                const MODE: &str = "production";
            }
        }
        assert_eq!(MODE, "testing");
    }
}
