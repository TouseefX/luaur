use luaur_vm::functions::lua_clock::lua_clock;

#[allow(clippy::empty_loop)]
pub fn profiler_loop() {
    // This is a native-only CLI profiler loop that relies on process-global state
    // (gProfiler, exit flag, frequency, ticks/samples counters, and callback hooks).
    //
    // That state is defined in the C++ CLI profiler implementation and is not part of
    // the provided translated Rust context for this one-shot item.
    //
    // Implementations of the referenced global state and callback wiring must be
    // provided by other translated items before this can be made functional.
    let _ = lua_clock();
    loop {
        std::thread::yield_now();
    }
}
