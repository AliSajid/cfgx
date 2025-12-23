#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Generate JSON license report using cargo-about"
#USAGE arg "[output_file]" help="Path to output JSON file" default="licenses_report.json"

# ============================================================================
# GENERATE LICENSE REPORT (JSON FORMAT)
# ============================================================================
# This script uses `cargo about` to generate a comprehensive JSON report of
# all dependencies and their associated licenses for the cfgx crate.
#
# The output is a machine-readable JSON file containing:
# - Complete dependency tree with version information
# - License text and metadata for each dependency
# - SPDX license identifiers where available
# - Repository and homepage URLs
# - Authors and copyright holders
#
# The JSON is formatted with:
# - Sorted keys for consistent output and easier diffing
# - 4-space indentation for readability
# - Unix line endings (LF) via dos2unix normalization
#
# PREREQUISITES:
# - rustc and cargo must be installed
# - cargo-about must be installed (cargo install cargo-about)
# - dos2unix must be available for line ending normalization
# - jq must be installed for JSON formatting
#
# USAGE:
#   mise run generate_about_json                    # Uses default: licenses_report.json
#   mise run generate_about_json custom_report.json # Saves to custom path
#
# OUTPUT:
#   The script generates a JSON file at the specified location containing
#   the complete license report. This file is suitable for:
#   - Automated license compliance checking
#   - Integration with CI/CD pipelines
#   - Feeding into other tooling for further processing
# ============================================================================

# Enable strict error handling:
# -e: Exit immediately if any command fails
# -u: Treat unset variables as errors
# -o pipefail: Fail if any command in a pipeline fails
# +x: Disable command echoing for cleaner output
set -euo pipefail
set +x

# ============================================================================
# STEP 1: Verify Rust Toolchain Installation
# ============================================================================
# Check that the Rust compiler (rustc) is available in PATH.
# This is required as cargo-about needs the Rust toolchain to analyze
# the dependency graph and resolve license information.

if ! command -v rustc &>/dev/null; then
    echo "ERROR: rustc (Rust compiler) could not be found in PATH"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check that the Cargo package manager is available.
# Cargo is needed to run cargo-about and manage the dependency tree.
if ! command -v cargo &>/dev/null; then
    echo "ERROR: cargo (Rust package manager) could not be found in PATH"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# ============================================================================
# STEP 2: Verify cargo-about Installation
# ============================================================================
# cargo-about is a third-party tool that analyzes Cargo.toml and generates
# license reports. It must be installed separately from the standard Rust
# toolchain. Install with: cargo install cargo-about
if ! command -v cargo-about &>/dev/null; then
    echo "ERROR: cargo-about could not be found in PATH"
    echo "Please install it with: cargo install cargo-about"
    exit 1
fi

# ============================================================================
# STEP 3: Verify jq Installation
# ============================================================================
# jq is a command-line JSON processor used to format and sort the output.
# This ensures consistent, readable output that's easy to diff in version control.
if ! command -v jq &>/dev/null; then
    echo "ERROR: jq (JSON processor) could not be found in PATH"
    echo "Please install it with: brew install jq (macOS) or apt-get install jq (Linux)"
    exit 1
fi

# ============================================================================
# STEP 4: Verify dos2unix Installation
# ============================================================================
# dos2unix normalizes line endings to Unix format (LF instead of CRLF).
# This prevents line ending issues across different operating systems.
if ! command -v dos2unix &>/dev/null; then
    echo "ERROR: dos2unix could not be found in PATH"
    echo "Please install it with: brew install dos2unix (macOS) or apt-get install dos2unix (Linux)"
    exit 1
fi

# ============================================================================
# STEP 5: Generate the JSON License Report
# ============================================================================
# Process flow:
# 1. cargo about generate --format json --frozen
#    - Generates the license report in JSON format
#    - --frozen ensures Cargo.lock is not modified (reproducible builds)
# 2. jq --sort-keys --indent 4 -r
#    - Sorts JSON keys alphabetically for consistent output
#    - Indents with 4 spaces for readability
#    - -r outputs raw strings (no extra JSON escaping)
# 3. > "$OUTPUT_FILE"
#    - Redirects output to the specified file

# Get output file path from argument or use default
OUTPUT_FILE="${usage_output_file:-licenses_report.json}"

echo "Generating JSON license report..."
echo "Output file: $OUTPUT_FILE"

# Generate the report with piped formatting
cargo about generate --format json --frozen | jq --sort-keys --indent 4 -r >"$OUTPUT_FILE"

# Apply line ending normalization to ensure Unix LF format
dos2unix "$OUTPUT_FILE" 2>/dev/null || true

echo "✓ JSON license report generated successfully: $OUTPUT_FILE"

# Re-enable command echoing for debugging if needed
set -x
