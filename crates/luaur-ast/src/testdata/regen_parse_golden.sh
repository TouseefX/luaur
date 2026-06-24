#!/usr/bin/env bash
# Regenerate the parser oracle golden files from the real Luau C++ parser.
# Run from anywhere. Requires clang++ and the vendored `luau/` sources.
#
# Builds testdata/parse_oracle.cpp against the real Ast module and dumps a
# canonical structural AST for each testdata/parse_*.luau fixture. The Rust
# port's `parser_oracle_test` must reproduce these byte-for-byte.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
HERE="$ROOT/translation/skeleton/crates/luau-ast/src/testdata"
BIN=/tmp/luau_parse_oracle

clang++ -std=c++17 -O0 \
  -I "$ROOT/luau/Ast/include" -I "$ROOT/luau/Common/include" \
  "$HERE/parse_oracle.cpp" \
  "$ROOT/luau/Ast/src/Parser.cpp" \
  "$ROOT/luau/Ast/src/Ast.cpp" \
  "$ROOT/luau/Ast/src/Cst.cpp" \
  "$ROOT/luau/Ast/src/Lexer.cpp" \
  "$ROOT/luau/Ast/src/Allocator.cpp" \
  "$ROOT/luau/Ast/src/Confusables.cpp" \
  "$ROOT/luau/Ast/src/Location.cpp" \
  "$ROOT/luau/Common/src/StringUtils.cpp" \
  -o "$BIN"

for fx in "$HERE"/parse_*.luau; do
  "$BIN" "$fx" > "${fx%.luau}.golden" || {
    echo "WARNING: $fx produced parse errors (see above)" >&2
  }
done
echo "regenerated parser goldens for: $(ls "$HERE"/parse_*.luau)"
