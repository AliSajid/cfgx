#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Generate Markdown license report using cargo-about with custom template"
#USAGE arg "<template_file>" help="Path to Handlebars template file (e.g., meta/licenses.hbs)"
#USAGE arg "[output_file]" help="Path to output Markdown file" default="licenses_report.md"

# ============================================================================
# GENERATE LICENSE REPORT (MARKDOWN FORMAT)
# ============================================================================
# This script uses `cargo about` to generate a human-readable Markdown report
# of all dependencies and their associated licenses for the cfgx crate.
#
# The output uses a custom Handlebars template to format the license information
# in a way that's suitable for documentation, README files, or compliance reports.
#
# The Markdown report typically includes:
# - Table of contents of all dependencies
# - Dependency name, version, and license type
# - Full license text for each dependency
# - Links to repositories and homepages
# - Formatted sections for easy navigation
#
# Text processing pipeline:
# - Handlebars template rendering by cargo-about
# - Tab expansion to 4 spaces via gexpand for consistent indentation
# - Line ending normalization to Unix LF via dos2unix
#
# PREREQUISITES:
# - rustc and cargo must be installed
# - cargo-about must be installed (cargo install cargo-about)
# - gexpand (GNU expand) for tab conversion
# - dos2unix for line ending normalization
# - A valid Handlebars template file (typically in meta/ directory)
#
# USAGE:
#   mise run generate_about_md meta/licenses.hbs                    # Default output
#   mise run generate_about_md meta/licenses.hbs custom_report.md   # Custom output
#
# TEMPLATE FORMAT:
#   The template file should be in Handlebars format (.hbs) and can use
#   cargo-about's template variables to format the license data.
#   See: https://github.com/EmbarkStudios/cargo-about#templates
#
# OUTPUT:
#   A formatted Markdown file suitable for:
#   - Including in project documentation
#   - Publishing on project websites
#   - Providing to legal/compliance teams
#   - Distributing with software releases
# ============================================================================

# Enable strict error handling:
# -e: Exit immediately if any command fails
# -u: Treat unset variables as errors
# -o pipefail: Fail if any command in a pipeline fails
set -euo pipefail

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
# STEP 3: Verify gexpand Installation
# ============================================================================
# gexpand (GNU expand) converts tabs to spaces for consistent formatting.
# This is part of GNU coreutils. On macOS, install with: brew install coreutils
if ! command -v gexpand &>/dev/null; then
    echo "ERROR: gexpand (GNU expand) could not be found in PATH"
    echo "Please install GNU coreutils:"
    echo "  macOS: brew install coreutils"
    echo "  Linux: Usually pre-installed, or use 'expand' instead of 'gexpand'"
    exit 1
fi

# ============================================================================
# STEP 4: Verify dos2unix Installation
# ============================================================================
# dos2unix normalizes line endings to Unix format (LF instead of CRLF).
# This prevents line ending issues across different operating systems.
if ! command -v dos2unix &>/dev/null; then
    echo "ERROR: dos2unix could not be found in PATH"
    echo "Please install it with:"
    echo "  macOS: brew install dos2unix"
    echo "  Linux: apt-get install dos2unix"
    exit 1
fi

# ============================================================================
# STEP 5: Validate Required Template File Argument
# ============================================================================
# The Handlebars template file is required to format the output.
# This file defines the structure and layout of the Markdown report.
# Typical location: meta/licenses.hbs

# Get template file from mise argument
TEMPLATE_FILE="${usage_template_file:-}"

if [ -z "$TEMPLATE_FILE" ]; then
    echo "ERROR: No template file was provided"
    echo "USAGE: mise run generate_about_md <template_file> [output_file]"
    echo "Example: mise run generate_about_md meta/licenses.hbs licenses_report.md"
    exit 1
fi

# Verify the template file exists and is readable
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "ERROR: Template file not found: $TEMPLATE_FILE"
    echo "Please ensure the template file exists and the path is correct."
    exit 1
fi

if [ ! -r "$TEMPLATE_FILE" ]; then
    echo "ERROR: Template file is not readable: $TEMPLATE_FILE"
    echo "Please check file permissions."
    exit 1
fi

# ============================================================================
# STEP 6: Generate the Markdown License Report
# ============================================================================
# Process flow:
# 1. cargo about generate --format handlebars --frozen <template>
#    - Generates the license report using the custom Handlebars template
#    - --frozen ensures Cargo.lock is not modified (reproducible builds)
# 2. gexpand -t 4
#    - Converts any tabs in the output to 4 spaces for consistent formatting
#    - Ensures consistent indentation across different editors
# 3. dos2unix
#    - Normalizes line endings to Unix LF format
#    - Prevents CRLF issues when editing on different platforms
# 4. > "$OUTPUT_FILE"
#    - Redirects the processed output to the specified file

# Get output file path from argument or use default
OUTPUT_FILE="${usage_output_file:-licenses_report.md}"

echo "Generating Markdown license report..."
echo "Template file: $TEMPLATE_FILE"
echo "Output file: $OUTPUT_FILE"

# Generate the report with piped text processing
cargo about generate --format handlebars --frozen "$TEMPLATE_FILE" | gexpand -t 4 | dos2unix >"$OUTPUT_FILE"

echo "✓ Markdown license report generated successfully: $OUTPUT_FILE"
