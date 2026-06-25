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
| fib(35)              | 650 (1.4x)  | 449 (1.0x)  | 471      | 418       | 421         | 58 (0.1x)   |
| nbody (500k steps)   | 666 (1.4x)  | 740 (1.6x)  | 470      | 500       | 668         | 43 (0.1x)   |
| mandelbrot 800²      | 1329 (1.0x) | 1123 (0.8x) | 1343     | 1331      | 1085        | 1256        |
| matmul 200³          | 151 (1.9x)  | 127 (1.6x)  | 82       | 81        | 109         | 13 (0.2x)   |
| tablesort 200k       | 27 (1.0x)   | 71 (2.8x)   | 26       | 25        | 62          | 50          |
| strings 200k         | 58 (1.1x)   | 64 (1.2x)   | 55       | 55        | 71          | 34          |

### Average across all benchmarks

Geometric mean of the per-benchmark time ratios (the correct way to average
speed ratios), with **luaur as the baseline**:

| Implementation            | Engine             | Average vs luaur       |
|---------------------------|--------------------|------------------------|
| **luaur**                 | Luau, pure Rust    | 1.00× (baseline)       |
| C++ luau (reference)      | Luau, C++          | **1.26× faster**       |
| mlua → luau               | Luau, C via FFI    | 1.27× faster           |
| mlua → Lua 5.4 (PUC-Rio)  | Lua 5.4, C         | ~parity (1.01× slower) |
| tsuki                     | Lua 5.4, pure Rust | 1.08× slower           |
| mlua → LuaJIT             | LuaJIT, JIT        | 3.5× faster            |

Averaged across these six workloads:

- luaur runs at **~0.79× the speed of the reference C++ Luau** (luau ~1.26×
  faster) — a strong result for a faithful, JIT-free pure-Rust port. `mlua→luau`
  lands in the same place, confirming the C engine behaves identically via FFI.
- luaur is **on par with stock PUC-Rio Lua 5.4** and **~1.08× faster than tsuki**,
  the other pure-Rust Lua VM — competitive with both the canonical C interpreter
  and the other Rust interpreter.
- **LuaJIT is ~3.5× faster** overall (and 5–15× on tight numeric loops) — the
  tracing-JIT ceiling that no plain interpreter here approaches.

Per workload, luaur ranges from parity (mandelbrot, tablesort, strings), to ~1.4×
(recursion-heavy fib/nbody), to ~1.9× worst case (matmul) versus C++ Luau.

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
