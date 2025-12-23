#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# A script that uses `cargo about` to generate a human-readable summary of
# the current crate's dependencies and their licenses and saves them
# to a markdown file.

# The script is intended to be run from the root of the crate.
# It needs one required parameters which will be the path
# to the handlebars template file. If no parameter is given,
# the script will exit with an error.
# It can take an optional parameter which will be the path
# to the markdown file to be generated. If no parameter is given,
# the file will be generated in the root of the crate.
#

set -euo pipefail
set +x

## Step 1: Check that rustc and cargo are installed

if ! command -v rustc &>/dev/null; then
    echo "rustc could not be found"
    exit
fi

if ! command -v cargo &>/dev/null; then
    echo "cargo could not be found"
    exit
fi

## Step 2: Check that cargo-about is installed

if ! command -v cargo-about &>/dev/null; then
    echo "cargo-about could not be found"
    exit
fi

## Step 3: Check that the normalization command is installed

if ! command -v dos2unix &>/dev/null; then
    echo "dos2unix could not be found"
    exit
fi

## Step 4: Generate the markdown file

OUTPUT_FILE=${1:-licenses_report.json}
cargo about generate --format json --frozen | jq --sort-keys --indent 4 -r >"$OUTPUT_FILE"
set -x
