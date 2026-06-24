pub fn get_timestamp() -> f64 {
    // `wasm32-unknown-unknown` has no clock backend: `std::time::SystemTime::now`
    // panics there ("time not implemented on this platform"). This timestamp is
    // only used for the type checker's timing instrumentation (analogous to the
    // `TimeTrace` no-ops), not for any correctness decision, so on wasm it
    // reports a fixed value instead of panicking.
    #[cfg(target_arch = "wasm32")]
    {
        0.0
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = std::time::SystemTime::now();
        let duration = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| e.duration());
        duration.as_secs_f64()
    }
}
