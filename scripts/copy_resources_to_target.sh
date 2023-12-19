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
declare is_web_build="$2"

if [ "$target" != "debug" ] && [ "$target" != "release" ]; then
  target="debug"
fi

if [ "$is_web_build" -eq 1 ]; then
    cp -fR "config" "target/wasm32-unknown-unknown/$target/out"
    cp -fR "assets" "target/wasm32-unknown-unknown/$target/out"
    cp -f "web/index.html" "target/wasm32-unknown-unknown/$target/out"
  else
    cp -fR "config" "target/$target"
    cp -fR "assets" "target/$target"
fi


