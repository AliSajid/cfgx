#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# Comprehensive test script for cfgx
# Tests all code paths using feature flags - works on any platform

set -e # Exit on error

echo "================================"
echo "CFGX Comprehensive Test Suite"
echo "Testing all paths via features"
echo "================================"
echo ""

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

run_test() {
    echo -e "${BLUE}Running:${NC} $1"
    eval "$1"
    echo -e "${GREEN}✓ Passed${NC}"
    echo ""
}

# 1. Basic tests without any features (default paths)
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

# 3. Platform simulation features
echo "=== Phase 3: Platform Simulation ==="
run_test "cargo test --no-default-features --features platform_a"
run_test "cargo test --no-default-features --features platform_b"
run_test "cargo test --no-default-features --features os_unix"
run_test "cargo test --no-default-features --features os_windows"

# 4. Architecture simulation
echo "=== Phase 4: Architecture Simulation ==="
run_test "cargo test --no-default-features --features arch_64"
run_test "cargo test --no-default-features --features arch_32"

# 5. Option features
echo "=== Phase 5: Option Features ==="
run_test "cargo test --no-default-features --features opt1"
run_test "cargo test --no-default-features --features opt2"
run_test "cargo test --no-default-features --features 'opt1,opt2'"

# 6. Combined feature tests (common realistic combinations)
echo "=== Phase 6: Feature Combinations ==="
run_test "cargo test --features 'f64_precision,high_precision'"
run_test "cargo test --features 'simd,large_buffers'"
run_test "cargo test --features 'experimental,extended'"
run_test "cargo test --features 'compact_genome,f64_precision'"
run_test "cargo test --features 'high_precision_stats,f64_precision'"
run_test "cargo test --features 'os_unix,arch_64'"
run_test "cargo test --features 'os_windows,arch_32'"

# 7. Multi-feature combinations for complex paths
echo "=== Phase 7: Complex Multi-Feature Combinations ==="
run_test "cargo test --features 'high_precision,extended' test_feature_combinations"
run_test "cargo test --features 'high_precision' test_feature_combinations"
run_test "cargo test --features 'extended' test_feature_combinations"
run_test "cargo test --no-default-features test_feature_combinations"

# 8. All features enabled (maximal path)
echo "=== Phase 8: All Features ==="
run_test "cargo test --all-features"

# 9. Specific test categories
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

# 10. Real-world tests with specific feature paths
echo "=== Phase 10: Real-World Scenario Paths ==="
echo -e "${YELLOW}Testing bioinformatics precision paths...${NC}"
run_test "cargo test test_bioinformatics_precision --features f64_precision"
run_test "cargo test test_bioinformatics_precision --no-default-features"

echo -e "${YELLOW}Testing platform I/O paths...${NC}"
run_test "cargo test test_platform_file_io --features os_unix"
run_test "cargo test test_platform_file_io --features os_windows"
run_test "cargo test test_platform_file_io --no-default-features"

echo -e "${YELLOW}Testing algorithm selection paths...${NC}"
run_test "cargo test test_algorithm_selection --features simd"
run_test "cargo test test_algorithm_selection --no-default-features"

echo -e "${YELLOW}Testing genomic data paths...${NC}"
run_test "cargo test test_genomic_data_representation --features compact_genome"
run_test "cargo test test_genomic_data_representation --no-default-features"

echo -e "${YELLOW}Testing statistical backend paths...${NC}"
run_test "cargo test test_statistical_backends --features high_precision_stats"
run_test "cargo test test_statistical_backends --no-default-features"

echo -e "${YELLOW}Testing allocation strategy paths...${NC}"
run_test "cargo test test_allocation_strategies --features large_buffers"
run_test "cargo test test_allocation_strategies --no-default-features"

# 11. Debug vs Release (different cfg paths)
echo "=== Phase 11: Build Profiles ==="
run_test "cargo test"           # Debug mode (debug_assertions = true)
run_test "cargo test --release" # Release mode (debug_assertions = false)
run_test "cargo test test_debug_release_config"
run_test "cargo test test_debug_release_config --release"

# 12. Documentation tests
echo "=== Phase 12: Documentation Tests ==="
run_test "cargo test --doc"

# 13. Check for no_std compatibility
echo "=== Phase 13: no_std Compatibility ==="
run_test "cargo build --no-default-features"
echo -e "${GREEN}✓ no_std build successful${NC}"
echo ""

# 14. Test both macro rules
echo "=== Phase 14: Macro Rule Coverage ==="
echo -e "${YELLOW}Testing block syntax rule...${NC}"
run_test "cargo test test_single_block_single_item"
run_test "cargo test test_multiple_separate_blocks"

echo -e "${YELLOW}Testing if/else syntax rule...${NC}"
run_test "cargo test test_simple_if_else_single_item"
run_test "cargo test test_multiple_if_else_blocks"

# 15. Exhaustive path coverage for specific tests
echo "=== Phase 15: Exhaustive Path Coverage ==="
echo -e "${YELLOW}Testing all paths for test_if_else_multiple_items...${NC}"
run_test "cargo test test_if_else_multiple_items --features arch_64"
run_test "cargo test test_if_else_multiple_items --no-default-features"

echo -e "${YELLOW}Testing all paths for test_multiple_if_else_blocks...${NC}"
run_test "cargo test test_multiple_if_else_blocks --features os_unix"
run_test "cargo test test_multiple_if_else_blocks --features 'os_unix,arch_64'"
run_test "cargo test test_multiple_if_else_blocks --features arch_64"
run_test "cargo test test_multiple_if_else_blocks --no-default-features"

echo "================================"
echo -e "${GREEN}All paths tested successfully!${NC}"
echo ""
echo "Summary:"
echo "  ✓ Both macro rules tested"
echo "  ✓ All features tested individually"
echo "  ✓ Common feature combinations tested"
echo "  ✓ All real-world scenarios tested"
echo "  ✓ Debug and release paths tested"
echo "  ✓ no_std compatibility verified"
echo ""
echo "Total test configurations: 60+"
echo "================================"
