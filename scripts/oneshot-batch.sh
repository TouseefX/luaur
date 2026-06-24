#!/usr/bin/env bash
# oneshot-batch.sh <captured_diffs_file>
# For every failing compiler test (diffs pre-captured), run oneshot that can fix
# EITHER the mis-ported Rust test expected OR the compiler source — whichever
# disagrees with the authoritative C++. Candidate source files are scoped by the
# test's first-diverging opcode so different-opcode tests touch disjoint files and
# run in parallel; same-opcode tests may conflict (zenpatch fail-safe) and re-run.
set -uo pipefail
CD="${1:?usage: oneshot-batch.sh <captured_diffs_file>}"
ROOT="/Users/pawel/Projects/luau-rs"
SK="$ROOT/translation/skeleton"
CPP="$ROOT/luau/tests/Compiler.test.cpp"
MAP="$SK/scripts/cpp_testmap.tsv"
PLANNER="${ONESHOT_PLANNER:-google/gemini-3.1-pro-preview}"
CONC="${CONC:-6}"
M="translation/skeleton/crates/luau-compiler/src/methods"
F="translation/skeleton/crates/luau-compiler/src/functions"
B="translation/skeleton/crates/luau-bytecode/src/methods"
WORK="$(mktemp -d)"; export WORK ROOT SK CPP MAP PLANNER M F B

# split captured diffs into per-test {left,right,firstdiff-opcode}
perl -e '
  my ($cd,$out)=@ARGV; open my $f,"<",$cd or die; my ($t,$l,$r);
  sub flush { my($t,$l,$r)=@_; return unless $t && defined $l && defined $r;
    my @la=split(/\\n/,$l); my @ra=split(/\\n/,$r); my $i=0;
    while($i<@la&&$i<@ra&&$la[$i] eq $ra[$i]){$i++;}
    my $line=($la[$i]//"")." || ".($ra[$i]//"");
    my ($op)= ("$la[$i] $ra[$i]" =~ /([A-Z][A-Z0-9_]{2,})/);
    open my $o,">","$out/$t.diff"; print $o "left:  $l\nright: $r\n"; close $o;
    open my $k,">","$out/$t.op"; print $k ($op//"NONE"); close $k;
  }
  while(<$f>){
    if(/panicked at .*tests\/(compiler_[a-z0-9_]+)\.rs/){ flush($t,$l,$r) if $t; $t=$1; $l=undef;$r=undef; }
    if(/^\s*left:\s*"(.*)"\s*$/){ $l=$1; }
    if(/^\s*right:\s*"(.*)"\s*$/){ $r=$1; }
  }
  flush($t,$l,$r) if $t;
' "$CD" "$WORK"
ls "$WORK"/*.diff 2>/dev/null | sed -E 's#.*/##; s/\.diff$//' > "$WORK/tests.txt"
echo "batch: $(wc -l < "$WORK/tests.txt") tests, conc=$CONC, planner=$PLANNER, work=$WORK"

candidates() { # echo source globs for an opcode
  case "$1" in
    ADD*|SUB*|MUL*|DIV*|MOD|IDIV|POW|ORK|AND|OR|MOVE|JUMPIF*|MINUS|NOT) echo "$M/compiler_compile_expr_binary.rs $M/compiler_compile_expr_and_or.rs $M/compiler_compile_expr_if_else_and_or.rs $F/fold_binary.rs $F/fold_constants.rs" ;;
    CALL*|NAMECALL|FASTCALL*|GETIMPORT|GETGLOBAL|SETGLOBAL|GETUPVAL|SETUPVAL) echo "$M/compiler_compile_expr_call.rs $F/analyze_builtins.rs $F/fold_builtin.rs $M/compiler_compile_expr_index_name.rs" ;;
    DUPCLOSURE|NEWCLOSURE|CAPTURE|CLOSEUPVALS) echo "$M/compiler_compile_expr_function.rs $M/compiler_should_share_closure.rs $B/bytecode_builder_dump_instruction.rs" ;;
    NEWTABLE|SETLIST|SETTABLE*|GETTABLE*|NEWCLASS*|DUPTABLE) echo "$M/compiler_compile_expr_table.rs $F/predict_table_size.rs" ;;
    LOADN|LOADK*|LOADB|LOADNIL) echo "$F/fold_constants.rs $M/compiler_compile_expr_binary.rs $M/compiler_compile_expr_constant.rs" ;;
    FORNPREP|FORNLOOP|FORGLOOP|FORGPREP*) echo "$M/compiler_compile_stat_for.rs $M/compiler_compile_unrolled_for.rs" ;;
    REMARK) echo "$B/bytecode_builder_dump_current_function.rs $M/compiler_compile_inlined_call.rs" ;;
    NONE) echo "$F/get_type.rs $F/get_base_type_string.rs $B/bytecode_builder_dump_type_info.rs $M/type_map_visitor_visit_types_alt_e.rs" ;;
    *) echo "$B/bytecode_builder_dump_instruction.rs" ;;
  esac
}
export -f candidates

run_one() {
  local T="$1"; local DIFF; DIFF=$(cat "$WORK/$T.diff"); local OP; OP=$(cat "$WORK/$T.op")
  local LINE; LINE=$(awk -F'\t' -v t="$T" '$1==t{print $2; exit}' "$MAP")
  [ -z "$LINE" ] && { echo "[$T] no C++ map — skip"; return; }
  local END; END=$(awk -v s="$LINE" 'NR>s && /^TEST_CASE/{print NR-1; exit}' "$CPP"); [ -z "$END" ] && END=$((LINE+90))
  local CPP_REF; CPP_REF=$(sed -n "${LINE},${END}p" "$CPP")
  local RT="translation/skeleton/crates/luau-unit-test/src/tests/${T}.rs"
  local SRC; SRC=$(candidates "$OP")
  local CARGS=( -c "$RT" ); for g in $SRC; do CARGS+=( -c "$g" ); done
  local P="A Luau compiler test '${T}' is FAILING. Diff — left=ACTUAL (current Rust compiler output), right=RUST-EXPECTED (the string in the test, which MAY be mis-ported):
${DIFF}

Authoritative upstream C++ test (R\"(...)\" expected blocks are the source of truth):
${CPP_REF}

The bug is in EXACTLY ONE place — hedge both:
- If RUST-EXPECTED DISAGREES with the C++ expected, the TEST was mis-ported: fix the test file ${RT} so its expected matches the C++ expected exactly (respect the leading-newline convention; tests compare format!(\"\\n{}\", actual) or \"\n\".to_string()+&actual against an expected starting with a newline).
- If RUST-EXPECTED already MATCHES the C++ expected, the COMPILER source is wrong: fix the provided luau-compiler/luau-bytecode source so ACTUAL matches it (faithful-port errors: wrong constant, missing/incorrect optimization branch, wrong opcode/operand, wrong dump encoding).
Make the MINIMAL change in the ONE correct place. NEVER change an expected to match the ACTUAL; only ever to match the C++ truth. Do not touch any file unrelated to this bug."
  echo "[$T] op=$OP C++:$LINE"
  ( cd "$ROOT" && oneshot --root "$ROOT" --no-ignore -m "$PLANNER" "${CARGS[@]}" "$P" >"$WORK/$T.out" 2>&1 )
  if grep -qE "changed [0-9]+ file" "$WORK/$T.out"; then echo "[$T] CHANGED: $(grep -oE '~ [^ ]+' "$WORK/$T.out" | sed -E 's#.*/##' | tr '\n' ' ')"; else echo "[$T] no-change/fail"; fi
}
export -f run_one
cat "$WORK/tests.txt" | xargs -P "$CONC" -I{} bash -c 'run_one "$@"' _ {}
echo "=== done. changed test files: $(cd "$ROOT"; git status --short | grep -c 'tests/compiler_') ; changed src files: $(cd "$ROOT"; git status --short | grep -cE 'luau-(compiler|bytecode)/src') ==="
echo "WORKDIR=$WORK"
