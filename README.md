# luaur

**A faithful, line-for-line translation of [Luau](https://github.com/luau-lang/luau) — Roblox's typed Lua — from C++17 to Rust.**

Not bindings. Not a reimplementation. The actual Luau compiler, virtual machine, and
type checker, ported to safe-by-default Rust and validated against Luau's **own** test
suite: **5,347 ported unit tests pass, and all 293 upstream conformance scripts run
byte-identically** on the Rust VM (against Luau commit [`8f33df9`](https://github.com/luau-lang/luau)).

```rust
// Compile and run Luau on a pure-Rust VM — no C, no FFI.
use luaur::{compile, eval};

// Run a script on a fresh VM (opens the standard library, like the `luau` CLI):
eval("print('hello from luaur')").unwrap();

// …or just compile source to bytecode:
let bytecode: Vec<u8> = compile("return 2 + 2").unwrap();
assert!(!bytecode.is_empty());
```

## Why this exists

Automated C++→Rust translation is an open problem. The published state of the art
(RustMap, EvoC2Rust, DARPA TRACTOR) tops out around **~13k lines of C at ~87%
equivalence with human patching** — their atomization breaks down before real scale.

luaur is **~205k lines of production C++17** (lexer, parser, bytecode compiler, register
VM, a full bidirectional type checker, native code generation, CLIs) translated to
**~420k lines of Rust**, with equivalence proven by two independent oracles instead of
spot checks:

1. **The maintainers' own test suite** — 5,347 unit tests ported and passing.
2. **A byte-exact bytecode differential** — programs compiled by C++ Luau and executed
   on the Rust VM produce identical results.

The interesting part isn't the graph-and-topo-sort skeleton (that's in the literature).
It's the **atomization and per-node context engineering** that let the obvious approach
survive to production scale as a *convergent* system — see
[`docs/TRANSLATION.md`](docs/TRANSLATION.md) for how it was actually built, the timeline,
the model economics, and the war stories.

## How idiomatic is it?

Body-to-body (imports, comments and blanks stripped), the port is **1.96×** the size of
the C++ — expansion you'd expect from making implicit C++ ownership explicit in Rust, not
from transliteration bloat. Pointers became `*mut T` where Luau's arena/GC model requires
it (faithfully), and ordinary value code became ordinary Rust.

```cpp
// C++ (Luau, VM/src/lvmexecute.cpp)
LuaTable* h = hvalue(ra);
const TValue* res = luaH_get(h, kv);
```
```rust
// Rust (luaur-vm)
let h = hvalue(ra);
let res = luaH_get(h, kv);
```

## The crates

luaur is published as independent crates so you can depend on exactly the layer you need:

| Crate | What it is |
|---|---|
| [`luaur`](crates/luaur) | Umbrella crate: re-exports every layer + `compile`/`eval` helpers |
| [`luaur-common`](crates/luaur-common) | Foundations: `SmallVector`, `DenseHashMap`, `Variant`, FastFlags |
| [`luaur-ast`](crates/luaur-ast) | Lexer, parser, AST |
| [`luaur-bytecode`](crates/luaur-bytecode) | Bytecode format + builder |
| [`luaur-compiler`](crates/luaur-compiler) | Luau source → bytecode compiler |
| [`luaur-code-gen`](crates/luaur-code-gen) | Native code generation (A64 / X64) |
| [`luaur-vm`](crates/luaur-vm) | The register VM + standard library |
| [`luaur-analysis`](crates/luaur-analysis) | Type checker / type inference |
| [`luaur-config`](crates/luaur-config) | `.luaurc` configuration |
| [`luaur-require`](crates/luaur-require) | Require-by-string module resolution |
| [`luaur-repl-cli`](crates/luaur-repl-cli) | Interactive REPL |
| [`luaur-analyze-cli`](crates/luaur-analyze-cli) | Standalone type-checker CLI |
| [`luaur-web`](crates/luaur-web) | `wasm32` bindings — run/type-check Luau in the browser |

(Plus `luaur-ast-cli`, `luaur-compile-cli`, `luaur-bytecode-cli`, `luaur-reduce-cli`, and `luaur-cli-lib`.)

## WebAssembly

The compiler, VM and type checker build for `wasm32-unknown-unknown` — the entire toolchain
runs client-side with no server. `luaur-web` exposes `run`/`check` entry points for an
in-browser playground:

```toml
luaur-web = { version = "0.1", features = ["wasm"] }
```

## Conformance scope

What is and isn't covered (and against which upstream commit) is stated precisely — no
blanket "perfect port" claims — in [`docs/CONFORMANCE.md`](docs/CONFORMANCE.md).

## License

MIT. luaur is a derivative translation of Luau (© Roblox Corporation) which derives from
Lua (© Lua.org, PUC-Rio); both upstream copyrights are preserved. See [`LICENSE`](LICENSE).
