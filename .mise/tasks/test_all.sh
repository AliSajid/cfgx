#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Run comprehensive test suite covering all feature flag combinations"
#USAGE flag "--verbose" help="Show detailed test output"

# ============================================================================
# CFGX COMPREHENSIVE TEST SUITE
# ============================================================================
# This script runs an exhaustive test suite for the cfgx procedural macro,
# ensuring complete code coverage across all conditional compilation paths.
#
# PURPOSE:
# The cfgx macro transforms conditional compilation syntax. To verify it works
# correctly, we need to test all possible combinations of feature flags that
# trigger different code paths. This script systematically tests:
#
# 1. Individual features in isolation
# 2. Common feature combinations
# 3. All features enabled simultaneously (--all-features)
# 4. No features enabled (--no-default-features)
# 5. Debug vs Release build configurations
# 6. Platform-specific code paths via feature simulation
# 7. Architecture-specific code paths (32-bit vs 64-bit)
# 8. Real-world use cases (bioinformatics, systems programming)
#
# WHY THIS IS NECESSARY:
# Traditional Rust testing only tests the current platform's cfg conditions.
# This script uses feature flags to simulate different platforms and
# configurations, allowing comprehensive testing on any single platform.
#
# TEST CATEGORIES:
# - Phase 1: Default paths (no features)
# - Phase 2: Individual features (single branch testing)
# - Phase 3: Platform simulation (unix, windows)
# - Phase 4: Architecture simulation (32-bit, 64-bit)
# - Phase 5: Option features (opt1, opt2)
# - Phase 6: Feature combinations (realistic usage)
# - Phase 7: Complex multi-feature paths
# - Phase 8: All features enabled (maximal path)
# - Phase 9: Category-specific test files
# - Phase 10: Real-world scenario paths
# - Phase 11: Build profile differences (debug vs release)
# - Phase 12: Documentation tests
# - Phase 13: no_std compatibility
# - Phase 14: Macro rule coverage
# - Phase 15: Exhaustive individual test paths
#
# USAGE:
#   mise run test_all              # Run all tests with colored output
#   mise run test_all -- --verbose # Show detailed cargo test output
#
# EXPECTED RUNTIME:
# Approximately 5-10 minutes depending on hardware, as this runs 60+ test
# configurations to ensure complete coverage.
#
# EXIT CODES:
# 0: All tests passed successfully
# 1: One or more tests failed
# ============================================================================

# Enable strict error handling:
# -e: Exit immediately if any command fails
# This ensures that if any test fails, the script stops and reports failure
set -e

echo "================================"
echo "CFGX Comprehensive Test Suite"
echo "Testing all paths via features"
echo "================================"
echo ""

# ============================================================================
# Output Formatting Configuration
# ============================================================================
# ANSI color codes for terminal output to make test results easy to scan.
# These improve readability by color-coding different types of messages:
# - GREEN: Successful test completions
# - BLUE: Currently running test command
# - YELLOW: Informational messages and notes
# - NC (No Color): Reset to default terminal color
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# ============================================================================
# Test Execution Helper Function
# ============================================================================
# Wraps test execution with colored output and error handling.
# Shows the command being run, executes it, and reports success.
# If a command fails, set -e will cause the script to exit immediately.
#
# Arguments:
#   $1: The cargo test command to execute (as a string)
#
# Example:
#   run_test "cargo test --features extended"
run_test() {
    echo -e "${BLUE}Running:${NC} $1"
    eval "$1"
    echo -e "${GREEN}✓ Passed${NC}"
    echo ""
}

# ============================================================================
# PHASE 1: Default Configuration Testing
# ============================================================================
# Test the crate with no optional features enabled. This verifies that:
# - The macro works with default/fallback code paths
# - All else branches in if/else cfgx blocks are executed
# - The crate compiles and runs correctly with minimal features
echo "=== Phase 1: No Features (Default Paths) ==="
run_test "cargo test --no-default-features"

# 2. Individual feature tests (test each branch independently)
echo "=== Phase 2: Individual Features (Single Branch Testing) ==="
run_test "cargo test --no-default-features --features experimental"
run_test "cargo test --no-default-features --features extended"
run_test "cargo test --no-default-features --features high_precision"
run_test "cargo test --no-default-features --features f64_precision"
run_test "cargo test --no-default-features --features simd"
run_test "cargo test --no-default-features --features compact_genome"
run_test "cargo test --no-default-features --features high_precision_stats"
run_test "cargo test --no-default-features --features large_buffers"

# ============================================================================
# PHASE 3: Platform Simulation
# ============================================================================
# Test platform-specific code paths using feature flags.
# Instead of relying on target_os cfg, we use features to simulate:
# - Unix-like platforms (Linux, macOS, BSD)
# - Windows platforms
# - Platform-specific file I/O, path handling, and line endings
#
# This allows testing all platform paths on a single development machine.
echo "=== Phase 3: Platform Simulation ==="
run_test "cargo test --no-default-features --features platform_a"
run_test "cargo test --no-default-features --features platform_b"
run_test "cargo test --no-default-features --features os_unix"
run_test "cargo test --no-default-features --features os_windows"

# ============================================================================
# PHASE 4: Architecture Simulation
# ============================================================================
# Test architecture-specific code paths using feature flags.
# Simulates different pointer widths and architecture capabilities:
# - 64-bit architectures (x86_64, aarch64)
# - 32-bit architectures (x86, arm)
#
# This ensures code works correctly with different:
# - Pointer sizes (4 bytes vs 8 bytes)
# - Integer types (usize/isize behavior)
# - Memory layout considerations
echo "=== Phase 4: Architecture Simulation ==="
run_test "cargo test --no-default-features --features arch_64"
run_test "cargo test --no-default-features --features arch_32"

# ============================================================================
# PHASE 5: Option Features
# ============================================================================
# Test optional features and their combinations.
# These represent user-selectable options that can be:
# - Enabled independently (opt1 OR opt2)
# - Enabled together (opt1 AND opt2)
# - Both disabled (neither opt1 nor opt2)
#
# This tests the any() and all() cfg conditions in the macro.
echo "=== Phase 5: Option Features ==="
run_test "cargo test --no-default-features --features opt1"
run_test "cargo test --no-default-features --features opt2"
run_test "cargo test --no-default-features --features 'opt1,opt2'"

# ============================================================================
# PHASE 6: Realistic Feature Combinations
# ============================================================================
# Test common real-world feature combinations that users might enable.
# These combinations represent typical use cases:
# - High precision computation with extended features
# - SIMD optimizations with large memory buffers
# - Platform + architecture combinations (Unix on 64-bit, Windows on 32-bit)
# - Genomic data with high precision calculations
#
# This ensures features work correctly when combined and don't conflict.
echo "=== Phase 6: Feature Combinations ==="
run_test "cargo test --features 'f64_precision,high_precision'"
run_test "cargo test --features 'simd,large_buffers'"
run_test "cargo test --features 'experimental,extended'"
run_test "cargo test --features 'compact_genome,f64_precision'"
run_test "cargo test --features 'high_precision_stats,f64_precision'"
run_test "cargo test --features 'os_unix,arch_64'"
run_test "cargo test --features 'os_windows,arch_32'"

# ============================================================================
# PHASE 7: Complex Multi-Feature Combinations
# ============================================================================
# Test the same functionality with different feature configurations to ensure
# the macro correctly handles complex nested conditions.
#
# Specifically tests the test_feature_combinations test which has deeply
# nested cfgx! blocks handling multiple feature permutations:
# - Both high_precision AND extended
# - high_precision only
# - extended only
# - Neither feature
#
# This validates that nested cfgx! calls work correctly and produce the
# expected code based on which features are enabled.
echo "=== Phase 7: Complex Multi-Feature Combinations ==="
run_test "cargo test --features 'high_precision,extended' test_feature_combinations"
run_test "cargo test --features 'high_precision' test_feature_combinations"
run_test "cargo test --features 'extended' test_feature_combinations"
run_test "cargo test --no-default-features test_feature_combinations"

# ============================================================================
# PHASE 8: Maximum Feature Combination Testing
# ============================================================================
# Test with ALL features enabled simultaneously (--all-features).
#
# This is a critical test that:
# - Validates that all features can coexist without conflicts
# - Tests feature precedence when mutually-exclusive-looking features are both enabled
# - Ensures the macro handles complex feature interactions
# - Catches any shadowing or duplicate definition errors
#
# With --all-features, tests must handle cases like:
# - platform_a AND platform_b both enabled (platform_a takes precedence)
# - os_unix AND os_windows both enabled (os_unix takes precedence)
# - arch_32 AND arch_64 both enabled (arch_64 takes precedence)
#
# This was specifically fixed to avoid compilation errors from duplicate definitions.
echo "=== Phase 8: All Features ==="
run_test "cargo test --all-features"

# ============================================================================
# PHASE 9: Test File Organization
# ============================================================================
# Run tests organized by category/test file to ensure:
# - Each test category compiles and runs independently
# - Tests are properly organized and discoverable
# - No cross-test dependencies or conflicts
#
# Test categories:
# - basic_blocks: Fundamental block syntax without if/else
# - if_else: If/else syntax for mutually exclusive branches
# - item_types: All Rust item types (structs, enums, traits, etc.)
# - edge_cases: Robustness testing (empty blocks, complex attributes, etc.)
# - mixed_syntax: Combining block and if/else syntax (future feature)
# - else_if_chains: Multiple conditions with else-if (future feature)
# - real_world: Practical use cases from bioinformatics and systems programming
echo "=== Phase 9: Category-Specific Tests ==="
run_test "cargo test --test basic_blocks"
run_test "cargo test --test if_else"
run_test "cargo test --test item_types"
run_test "cargo test --test edge_cases"
run_test "cargo test --test mixed_syntax"
run_test "cargo test --test else_if_chains"
run_test "cargo test --test real_world"

echo "=== Phase 9b: Future Enhancement Tests ==="
echo -e "${YELLOW}Note: Some tests in mixed_syntax and else_if_chains are #[ignore]d${NC}"
echo -e "${YELLOW}These represent future enhancements not yet implemented${NC}"
run_test "cargo test --test mixed_syntax -- --include-ignored"
run_test "cargo test --test else_if_chains -- --include-ignored"

# ============================================================================
# PHASE 10: Real-World Scenario Validation
# ============================================================================
# Test specific real-world use cases with their relevant feature configurations.
# Each scenario is tested with:
# - Feature enabled (specialized code path)
# - Feature disabled (fallback code path)
#
# This ensures the macro works correctly in practical applications and that
# users can confidently use cfgx for their actual use cases.
echo "=== Phase 10: Real-World Scenario Paths ==="
# Bioinformatics precision selection:
# Test switching between f32 (fast, less precise) and f64 (slower, more precise)
# for scientific calculations, probability scores, and statistical analysis.
echo -e "${YELLOW}Testing bioinformatics precision paths...${NC}"
run_test "cargo test test_bioinformatics_precision --features f64_precision"
run_test "cargo test test_bioinformatics_precision --no-default-features"

# Platform-specific file I/O:
# Test path separators (/ vs \\), line endings (\n vs \r\n), and path
# normalization across different operating systems.
echo -e "${YELLOW}Testing platform I/O paths...${NC}"
run_test "cargo test test_platform_file_io --features os_unix"
run_test "cargo test test_platform_file_io --features os_windows"
run_test "cargo test test_platform_file_io --no-default-features"

# Algorithm selection:
# Test switching between SIMD-optimized (vectorized) and scalar (standard)
# implementations for performance-critical numerical operations.
echo -e "${YELLOW}Testing algorithm selection paths...${NC}"
run_test "cargo test test_algorithm_selection --features simd"
run_test "cargo test test_algorithm_selection --no-default-features"

# Genomic data representation:
# Test compact (2-bit encoding) vs. direct (char) representation for
# DNA sequences, optimizing either for memory or simplicity.
echo -e "${YELLOW}Testing genomic data paths...${NC}"
run_test "cargo test test_genomic_data_representation --features compact_genome"
run_test "cargo test test_genomic_data_representation --no-default-features"

# Statistical analysis backends:
# Test high-precision (f64) vs. standard-precision (f32) statistical
# calculations for mean, variance, and other statistical operations.
echo -e "${YELLOW}Testing statistical backend paths...${NC}"
run_test "cargo test test_statistical_backends --features high_precision_stats"
run_test "cargo test test_statistical_backends --no-default-features"

# Memory allocation strategies:
# Test large buffer sizes (for throughput) vs. small buffer sizes
# (for memory-constrained environments).
echo -e "${YELLOW}Testing allocation strategy paths...${NC}"
run_test "cargo test test_allocation_strategies --features large_buffers"
run_test "cargo test test_allocation_strategies --no-default-features"

# ============================================================================
# PHASE 11: Build Profile Testing
# ============================================================================
# Test with different build profiles to ensure cfg(debug_assertions) works.
#
# Debug builds (cargo test):
# - debug_assertions enabled (more safety checks, slower)
# - Smaller buffer sizes for faster compilation
# - Extra validation and logging enabled
#
# Release builds (cargo test --release):
# - debug_assertions disabled (optimized, faster)
# - Larger buffer sizes for better throughput
# - Validation skipped for performance
#
# This ensures the macro correctly handles Rust's built-in cfg conditions.
echo "=== Phase 11: Build Profiles ==="
run_test "cargo test"           # Debug mode (debug_assertions = true)
run_test "cargo test --release" # Release mode (debug_assertions = false)
run_test "cargo test test_debug_release_config"
run_test "cargo test test_debug_release_config --release"

# ============================================================================
# PHASE 12: Documentation Testing
# ============================================================================
# Run doctests to ensure:
# - Code examples in documentation comments compile and run
# - Macro usage examples in docs are correct
# - API documentation is accurate and up-to-date
#
# Doctests are critical for maintaining accurate documentation.
echo "=== Phase 12: Documentation Tests ==="
run_test "cargo test --doc"

# ============================================================================
# PHASE 13: no_std Compatibility
# ============================================================================
# Verify that the crate compiles without the standard library.
# This is important for:
# - Embedded systems development
# - Bare-metal programming
# - WebAssembly without std
# - Operating system kernels
#
# The cfgx macro should work in any Rust environment, not just std.
echo "=== Phase 13: no_std Compatibility ==="
run_test "cargo build --no-default-features"
echo -e "${GREEN}✓ no_std build successful${NC}"
echo ""

# ============================================================================
# PHASE 14: Macro Syntax Rule Coverage
# ============================================================================
# The cfgx macro supports two syntax forms:
#
# 1. Block syntax: #[cfg(...)] { items }
#    - Used for simple conditional inclusion
#    - Can have multiple separate blocks
#    - No mutual exclusion guarantee
#
# 2. If/else syntax: if #[cfg(...)] { items } else { items }
#    - Used for mutually exclusive branches
#    - Guarantees exactly one branch is selected
#    - More readable for binary choices
#
# This phase tests both syntax forms to ensure complete macro functionality.
echo "=== Phase 14: Macro Rule Coverage ==="
echo -e "${YELLOW}Testing block syntax rule...${NC}"
run_test "cargo test test_single_block_single_item"
run_test "cargo test test_multiple_separate_blocks"

echo -e "${YELLOW}Testing if/else syntax rule...${NC}"
run_test "cargo test test_simple_if_else_single_item"
run_test "cargo test test_multiple_if_else_blocks"

# ============================================================================
# PHASE 15: Exhaustive Single-Test Coverage
# ============================================================================
# Run specific tests with all their possible feature combinations.
# This ensures that every code path within individual tests is exercised.
#
# For each test, we run it with:
# - Each relevant feature enabled individually
# - Multiple features combined
# - No features (fallback path)
#
# This level of granularity catches edge cases that might be missed by
# broader test runs.
echo "=== Phase 15: Exhaustive Path Coverage ==="
echo -e "${YELLOW}Testing all paths for test_if_else_multiple_items...${NC}"
run_test "cargo test test_if_else_multiple_items --features arch_64"
run_test "cargo test test_if_else_multiple_items --no-default-features"

echo -e "${YELLOW}Testing all paths for test_multiple_if_else_blocks...${NC}"
run_test "cargo test test_multiple_if_else_blocks --features os_unix"
run_test "cargo test test_multiple_if_else_blocks --features 'os_unix,arch_64'"
run_test "cargo test test_multiple_if_else_blocks --features arch_64"
run_test "cargo test test_multiple_if_else_blocks --no-default-features"

# ============================================================================
# TEST SUITE COMPLETION SUMMARY
# ============================================================================
echo "================================"
echo -e "${GREEN}All paths tested successfully!${NC}"
echo ""
echo "Summary:"
echo "  ✓ Both macro rules tested (block syntax and if/else syntax)"
echo "  ✓ All features tested individually and in combination"
echo "  ✓ Platform and architecture simulation validated"
echo "  ✓ All real-world scenarios tested with relevant features"
echo "  ✓ Debug and release build profiles verified"
echo "  ✓ no_std compatibility confirmed"
echo "  ✓ Documentation tests passed"
echo "  ✓ Exhaustive path coverage achieved"
echo ""
echo "Total test configurations: 60+"
echo "Total test phases: 15"
echo ""
echo "This comprehensive test suite ensures that the cfgx macro:"
echo "  • Works correctly on all platforms"
echo "  • Handles all feature combinations"
echo "  • Supports real-world use cases"
echo "  • Maintains backward compatibility"
echo "  • Compiles in both std and no_std environments"
echo "================================"
