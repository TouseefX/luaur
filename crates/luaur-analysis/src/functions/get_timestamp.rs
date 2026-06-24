pub fn get_timestamp() -> f64 {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|e| e.duration());
    duration.as_secs_f64()
}
