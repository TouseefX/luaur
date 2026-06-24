#!/usr/bin/env bash
# oneshot-fix.sh <compiler_test_name> <candidate_src_file_or_glob> [more globs...]
# Capture a failing compiler test's bytecode diff + its C++ counterpart, then run
# oneshot pointed at the candidate Rust source file(s) to find & fix the bug.
# The candidate files are the EDIT targets; C++ reference is inlined read-only.
set -uo pipefail
T="$1"; shift
ROOT="/Users/pawel/Projects/luau-rs"
SK="$ROOT/translation/skeleton"
PLANNER="${ONESHOT_PLANNER:-google/gemini-3.1-pro-preview}"

cd "$SK"
LOG=$(CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 scripts/nextest-safe.sh -E "test(/${T}::${T}\$/)" 2>&1)
DIFF=$(printf '%s\n' "$LOG" | grep -E "^\s+(left|right):" | head -2)
if [ -z "$DIFF" ]; then echo "[$T] SKIP: no left/right diff (crash or already passing)"; exit 2; fi

RUST_TEST="translation/skeleton/crates/luau-unit-test/src/tests/${T}.rs"

# Map rust snake_case test -> C++ PascalCase TEST_CASE line via a cached table.
MAP="$SK/scripts/cpp_testmap.tsv"
CPP="$ROOT/luau/tests/Compiler.test.cpp"
if [ ! -f "$MAP" ] || [ "$CPP" -nt "$MAP" ]; then
  grep -nE 'TEST_CASE(_FIXTURE)?\(' "$CPP" | perl -ne 'if (/^(\d+):.*"([A-Za-z0-9_]+)"\)/){my($l,$n)=($1,$2);my $s=$n;$s=~s/([a-z0-9])([A-Z])/${1}_${2}/g;$s=~s/([A-Z]+)([A-Z][a-z])/${1}_${2}/g;$s=lc$s;print "compiler_$s\t$l\n";}' > "$MAP"
fi
CPP_LINE=$(awk -F'\t' -v t="$T" '$1==t{print $2; exit}' "$MAP")
CPP_REF=""
if [ -n "$CPP_LINE" ]; then
  # from CPP_LINE to the line before the NEXT TEST_CASE (whole test block)
  END=$(awk -v s="$CPP_LINE" 'NR>s && /^TEST_CASE/{print NR-1; exit}' "$CPP")
  [ -z "$END" ] && END=$((CPP_LINE+80))
  CPP_REF=$(sed -n "${CPP_LINE},${END}p" "$CPP")
else
  echo "[$T] WARNING: no C++ mapping found — refusing to run blind (would trust the possibly-mis-ported Rust expected)"; exit 3
fi

# build -c args from candidate globs (the edit targets) + the rust test (reference for what's compiled)
CARGS=( -c "$RUST_TEST" )
for g in "$@"; do CARGS+=( -c "$g" ); done

PROMPT="A Luau compiler bytecode test '${T}' is FAILING. assert_eq diff —
  left  = ACTUAL  : what the Rust compiler currently produces
  right = RUST-EXPECTED : the expected string in the Rust test (MAY BE MIS-PORTED)
${DIFF}

The UPSTREAM C++ test is the SOURCE OF TRUTH (its R\"(...)\" expected blocks are authoritative):
${CPP_REF}

Decide and fix, in this priority:
1. FIRST compare RUST-EXPECTED against the C++ expected. If they DIFFER, the Rust TEST was mis-ported — fix the test file (${RUST_TEST}) so its expected string matches the C++ expected EXACTLY (mind the leading-newline convention: the Rust test compares format!(\"\\n{}\", actual) or \"\n\"+actual against an expected that begins with \\n).
2. If RUST-EXPECTED already matches C++, then the COMPILER SOURCE is wrong — fix the provided luau-compiler/luau-bytecode source so ACTUAL matches the C++ expected. Faithful-port errors: wrong constant, missing/incorrect optimization branch, wrong opcode/operand, wrong dump-encoding.
Make the MINIMAL change. Never change an expected value to something the C++ does NOT say."

echo "[$T] oneshot planner=$PLANNER targets:$* "
oneshot --root "$ROOT" --no-ignore -m "$PLANNER" "${CARGS[@]}" "$PROMPT"
