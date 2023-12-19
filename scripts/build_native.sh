#!/usr/bin/env bash

#
# Copyright (c)  Sebastian Riga 2023.
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of this software
# and associated
# documentation files (the “Software”), to deal in the Software without restriction, including
# without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
# and/or sell copies of the Software, and to permit persons to whom the
# Software is furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all copies
# or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
# INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
# PURPOSE AND NONINFRINGEMENT.
# IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#

set -e

declare target="$1"

if [ "$target" != "debug" ] && [ "$target" != "release" ]; then
  target="debug"
  echo "No valid target specified, falling back to debug!"
fi

echo "Native release will be build for: $target"

if [ "$target" == "release" ]; then
    echo "Running unit tests..."
    cargo test
    echo "Starting release build..."
    cargo build --release
  else
    echo "Starting debug build..."
    cargo build
fi

echo "Copying resources to output dir..."

sh copy_resources_to_target.sh "$target" 0

echo "Done. Output can bound at: target/$target"
