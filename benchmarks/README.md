# luaur benchmarks

A small, reproducible benchmark suite comparing **luaur** (this pure-Rust Luau
port) against the reference C++ Luau and other Lua-family runtimes, on both
**execution speed** and **compilation speed**.

## Engines compared

| Engine        | Language        | Kind                          |
|---------------|-----------------|-------------------------------|
| **luaur**     | Luau            | pure Rust (this repo)         |
| tsuki         | Lua 5.4         | pure Rust                     |
| C++ luau      | Luau            | reference C++ interpreter     |
| mlua/luau     | Luau            | C Luau via Rust FFI           |
| mlua/lua5.4   | Lua 5.4         | C Lua via Rust FFI            |
| mlua/luajit   | LuaJIT (5.1)    | tracing JIT via Rust FFI      |

All engines run the **same** program source. Each program prints a single
checksum, and the harness verifies every engine produces the **identical**
checksum — so the timings are only trusted when the work was actually done
correctly. (They all agree.)

## Methodology

- Release/optimized builds throughout (`cargo build --release`; C++ Luau built
  `-DCMAKE_BUILD_TYPE=Release`; mlua C sources vendored at `-O3`).
- Wall-clock time of the whole process, median of 7 runs after 1 warm-up.
  Process start-up was measured at well under 10 ms for every engine, i.e.
  negligible against the workloads below.
- Programs (`programs/`) are plain Lua that is valid on Luau, Lua 5.4 and
  LuaJIT alike (no `//`, no bitwise ops, no type annotations); a Park–Miller
  RNG keeps integer math exact in both double and 64-bit-integer engines so the
  checksums match everywhere.
- luaur runs as a **bytecode interpreter** here (no native codegen).
- Numbers below are from one Apple Silicon laptop (macOS, arm64). Treat them as
  ratios and ballparks, not absolutes — rerun locally with `harness.py`.

## Runtime results

Median wall-clock **ms** (lower = faster); `(Nx)` is the slowdown vs C++ Luau.

| Benchmark            | luaur       | tsuki       | C++ luau | mlua/luau | mlua/lua5.4 | mlua/luajit |
|----------------------|-------------|-------------|----------|-----------|-------------|-------------|
| fib(35)              | 658 (1.4x)  | 455 (1.0x)  | 475      | 420       | 427         | 60 (0.1x)   |
| nbody (500k steps)   | 670 (1.4x)  | 739 (1.6x)  | 466      | 501       | 666         | 43 (0.1x)   |
| mandelbrot 800²      | 1337 (1.0x) | 1127 (0.8x) | 1333     | 1321      | 1084        | 1259        |
| matmul 200³          | 152 (1.9x)  | 126 (1.5x)  | 82       | 80        | 109         | 14 (0.2x)   |
| tablesort 200k       | 27 (1.0x)   | 72 (2.7x)   | 26       | 26        | 63          | 51          |
| strings 200k         | 56 (1.0x)   | 69 (1.3x)   | 55       | 58        | 70          | 33          |

Takeaways:

- **luaur vs C++ Luau:** within **1.0–1.9×** across the board — parity on the
  float/loop, sort and string workloads, ~1.4× on recursion-heavy `fib`/`nbody`,
  ~1.9× worst case on `matmul`. Good standing for a faithful pure-Rust port with
  no JIT, and `mlua/luau` confirms the C engine behaves the same via FFI.
- **luaur vs tsuki** (the other pure-Rust VM): a wash with trade-offs — luaur is
  markedly faster on `tablesort` (2.7×), and ahead on `nbody`/`strings`; tsuki is
  ahead on `fib`/`mandelbrot`/`matmul`. (They're different languages — Luau vs
  Lua 5.4 — running equivalent source.)
- **mlua/luajit** is the JIT ceiling: 5–15× faster on tight numeric loops, but no
  advantage (sometimes slower) on library-bound work like `table.sort`.

## Compilation speed

Compiling a generated **2.14 MB / 85,717-line** Luau file to bytecode
(`null` mode = compile + discard), median of 9 runs:

| Compiler             | median ms | MB/s | vs C++ |
|----------------------|-----------|------|--------|
| **luaur-compile**    | 131       | 16.4 | 0.77×  |
| C++ luau-compile     | 170       | 12.6 | 1.00×  |

luaur's compiler is **~1.3× faster** than the reference C++ Luau compiler on
this corpus (both produce identical, mutually-runnable bytecode).

## Reproduce

```sh
# 1. Build this repo's binaries
cargo build --release            # produces target/release/{luaur,luaur-compile}

# 2. (optional) build the comparison drivers
benchmarks/drivers/mlua-run/build.sh        # mlua-{luau,lua54,luajit}
(cd benchmarks/drivers/tsuki-run && cargo build --release)
# and build reference C++ Luau separately (cmake -DCMAKE_BUILD_TYPE=Release)

# 3. Point the harness at your binaries and run
cd benchmarks
cp engines.example.json engines.json   # edit paths
python3 harness.py                     # runtime + correctness

# 4. Compilation speed
python3 gen_big.py
../target/release/luaur-compile null big.luau   # time this vs luau-compile
```
