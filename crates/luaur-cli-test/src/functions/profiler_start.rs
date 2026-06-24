//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Function:Luau.CLI.Test:CLI/src/Profiler.cpp:100:profiler_start`
//! Source: `CLI/src/Profiler.cpp`
//! Graph edges:
//! - declared_by: source_file CLI/src/Profiler.cpp
//! - source_includes:
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//! - incoming:
//!   - declares <- source_file CLI/src/Profiler.cpp
//!   - calls <- function replMain (CLI/src/Repl.cpp)
//! - outgoing:
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - translates_to -> rust_item profilerStart

pub fn profiler_start() {
    // C++ `profilerStart(lua_State* L, int frequency)` records the sampling
    // frequency and lua_callbacks(L) into the process-global `gProfiler`, then
    // spawns the `profilerLoop` sampling thread. That global profiler state is
    // native-only CLI state and is not part of the translated Rust context
    // (mirroring the documented no-op siblings `profiler_stop`/`profiler_loop`).
    // `repl_main` already treats profilerStart as a no-op in this port, so there
    // is no call site here.
}
