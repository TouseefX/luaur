#!/usr/bin/env bash
# oneshot-cluster.sh <captured_diffs_file>
# Split failing compiler tests into RELATED batches and run one oneshot per batch.
# Each batch gets broad source globs and lets oneshot pick the relevant files itself
# (--search-files); C++ is authoritative reference; --partial lands the good hunks.
set -uo pipefail
CD="${1:?usage: oneshot-cluster.sh <captured_diffs_file>}"
ROOT="/Users/pawel/Projects/luau-rs"
SK="$ROOT/translation/skeleton"
CPP="$ROOT/luau/tests/Compiler.test.cpp"
PLANNER="${ONESHOT_PLANNER:-google/gemini-3.5-flash}"
# Filter must be SUPER cheap — it sweeps the whole source pool per batch.
# gemini-2.5-flash-lite = $0.10/M in (5x cheaper than gemini-3-flash).
FILTER="${ONESHOT_FILTER:-google/gemini-2.5-flash-lite}"
WORK="$(mktemp -d)"

# 1. per-test diffs
perl -e '
  my ($cd,$out)=@ARGV; open my $f,"<",$cd or die; my ($t,$l,$r);
  while(<$f>){
    if(/panicked at .*tests\/(compiler_[a-z0-9_]+)\.rs/){ $t=$1; }
    if(/^\s*left:\s*(.*)$/){ $l=$1; }
    if(/^\s*right:\s*(.*)$/){ $r=$1; if($t){ open my $o,">","$out/$t.diff"; print $o "  actual:   $l\n  expected: $r\n"; close $o; } }
  }
' "$CD" "$WORK"

# 2. assign each test to a RELATED batch by name keyword
classify() {
  case "$1" in
    *fold*|*const*|*arith*|*and_or*|*jump_fold*|*numeric_loop*|*builtin_extract*|*builtin_fold*|*custom_constant*) echo fold ;;
    *inline*) echo inline ;;
    *debug*|*line_info*|*remark*|*coverage*|*cost_model*) echo debug ;;
    *import*|*call*|*fastcall*|*namecall*|*builtin_arity*|*builtin_folding*|*mutable_globals*|*fake_import*) echo call ;;
    *capture*|*closure*|*upvalue*) echo closure ;;
    *loop*|*repeat*|*for_bytecode*) echo loop ;;
    *table*|*indexer*|*class*|*export*) echo table ;;
    *type*|*encoded_type*|*builtin_type*) echo type ;;
    *) echo misc ;;
  esac
}
for d in "$WORK"/*.diff; do t=$(basename "$d" .diff); b=$(classify "$t"); echo "$t" >> "$WORK/batch_$b.list"; done
echo "batches:"; for f in "$WORK"/batch_*.list; do echo "  $(basename "$f" .list | sed s/batch_//): $(wc -l < "$f")"; done

run_batch() {
  local B="$1"; local LIST="$WORK/batch_$B.list"; [ -f "$LIST" ] || return
  local PF="$WORK/prompt_$B.txt"; local CARGS=()
  {
    echo "Fix these failing Luau COMPILER tests (related group: $B). For each, left=ACTUAL (Rust compiler), right=RUST-EXPECTED (test string, may be mis-ported):"
    while read t; do
      echo; echo "### $t"; cat "$WORK/$t.diff"
      local ln; ln=$(awk -F'\t' -v x="$t" '$1==x{print $2;exit}' "$SK/scripts/cpp_testmap.tsv")
      [ -n "$ln" ] && { local end; end=$(awk -v s="$ln" 'NR>s&&/^TEST_CASE/{print NR-1;exit}' "$CPP"); echo "  -- C++ truth (Compiler.test.cpp:$ln):"; sed -n "${ln},${end}p" "$CPP" | sed 's/^/    /'; }
      CARGS+=( -c "translation/skeleton/crates/luau-unit-test/src/tests/$t.rs" )
    done < "$LIST"
    cat <<'EOF'

The upstream C++ test blocks above are the AUTHORITATIVE truth. For each failing test:
- If RUST-EXPECTED disagrees with the C++ expected, the TEST was mis-ported: fix the test file's expected to match C++ exactly (leading-newline convention: tests compare format!("\n{}", actual) or "\n".to_string()+&actual vs an expected starting with a newline).
- If RUST-EXPECTED matches the C++ expected, the COMPILER SOURCE is wrong: fix the relevant Rust file (use the C++ compiler under luau/Compiler/src, luau/Bytecode/src as reference).
ONLY edit files under translation/skeleton/. luau/ is READ-ONLY reference. Minimal correct changes; NEVER change an expected to match the ACTUAL, only to match C++.
EOF
  } > "$PF"
  echo "[$B] running oneshot ($(wc -l < "$LIST") tests)..."
  # Edit pool = Rust compiler + bytecode src (filter picks the relevant files) + the
  # batch's test files. C++ truth is inlined in the prompt, so no C++ src in the pool
  # (it would bloat the filter and must never be edited).
  ( cd "$ROOT" && oneshot --root "$ROOT" --no-ignore --partial --search-files \
      --filter-model "$FILTER" -m "$PLANNER" --max-tokens 90000 \
      -c "translation/skeleton/crates/luau-compiler/src" \
      -c "translation/skeleton/crates/luau-bytecode/src" \
      "${CARGS[@]}" \
      -p "$PF" > "$WORK/out_$B.log" 2>&1 )
  echo "[$B] $(grep -E 'changed [0-9]+ file|failed|truncated' "$WORK/out_$B.log" | head -1)"
}
export -f run_batch; export WORK ROOT SK CPP PLANNER FILTER

ls "$WORK"/batch_*.list | sed -E 's#.*/batch_##; s/\.list$//' | xargs -P 1 -I{} bash -c 'run_batch "$@"' _ {}
echo "=== cluster done. changed: test=$(cd "$ROOT"; git status --short|grep -c 'tests/compiler_') src=$(cd "$ROOT"; git status --short|grep -cE 'luau-(compiler|bytecode)/src') ==="
echo "WORKDIR=$WORK"
