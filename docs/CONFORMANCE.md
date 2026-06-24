# Conformance scope

This document states exactly what equivalence has been demonstrated, against which
upstream version, and what is explicitly out of scope. A scoped, falsifiable claim is
worth more than a blanket "perfect port."

## Upstream baseline

- **Source:** [luau-lang/luau](https://github.com/luau-lang/luau) at commit `8f33df9`.
- **Translated subsystems:** `Ast`, `Compiler`, `Bytecode`, `CodeGen`, `VM`, `Analysis`,
  `Common`, `Config`, `Require`, and the `CLI` tools — ~205k lines of C++17 (excluding tests).

## What passes

| Oracle | Result |
|---|---|
| Ported unit suite (Luau's own doctest tests, translated to `#[test]`) | **5,347 pass / 0 fail** |
| Upstream conformance scripts (`tests/conformance/*.luau`) on the Rust VM | **293 / 293** |
| Byte-exact bytecode differential (C++-compiled bytecode run on the Rust VM) | identical results |

The two oracles are independent: the unit suite checks each subsystem's behavior in
isolation; the conformance + bytecode differential checks the integrated compiler→VM
pipeline end-to-end against the reference implementation.

## What is out of scope (and why)

- **Native code generation (JIT) execution.** `luaur-code-gen` translates the A64 and X64
  instruction builders, assemblers and IR lowering, and these are tested at the
  encoder/assembler level (byte-exact instruction encoding). End-to-end execution of
  JIT-compiled native code is **not** part of the differential run — the bytecode
  interpreter is the execution oracle. The `--codegen` CLI flag wires the codegen entry
  points but cannot thread two upstream `CompilationOptions` (`CodeGen_ColdFunctions`,
  perf-log) because the current public codegen API does not expose the options-bearing
  compile overload.
- **The new constraint-based type solver.** Luau is mid-migration from the old type
  solver to a new constraint solver; both exist in the source. The **old solver is the
  validated path**. New-solver internals are ported where they have a concrete upstream
  body, but new-solver-only behaviors are gated behind a test guard
  (`DOES_NOT_PASS_NEW_SOLVER_GUARD`) and are not part of the green bar. This mirrors
  upstream, where the new solver is itself work-in-progress.
- **Line editor.** The interactive REPL uses the [`rustyline`](https://crates.io/crates/rustyline)
  crate rather than a translation of Luau's vendored `isocline` C library — the idiomatic
  Rust analog of an external dependency. REPL semantics (completion, multi-line input,
  history) are preserved.
- **Profiling / external tooling.** `TimeTrace` instrumentation and valgrind/callgrind
  client hooks are faithful no-ops outside their respective environments, exactly as the
  C++ macros compile to no-ops in standard builds.

## Out of scope by construction: the ergonomic API

The `luaur-rt` crate (the mlua-style `Lua`/`Value`/`UserData` API) has **no upstream C++
counterpart** — Luau exposes only a C API. There is therefore nothing to "conform" to; it
is a Rust-native addition, validated by its own unit tests (not the conformance suite).
The conformance claims here are about the translated *engine* only.

## Known gaps

- **Documented C++-isms:** a small number of C++ special-member functions (copy/move
  constructors, copy-assignment), template-generic helpers superseded by monomorphized
  Rust callers, and one `_DEPRECATED` function have no Rust call site. Rather than leave
  bare `todo!()`s that read as unfinished work, each is an `unreachable!()`/`unimplemented!()`
  carrying a one-line reason. They are unreachable by construction, which is why the full
  suite is green.

## Reproducing

```sh
cargo nextest run --workspace      # 5,347 + 293 + CLI integration, all green
```
