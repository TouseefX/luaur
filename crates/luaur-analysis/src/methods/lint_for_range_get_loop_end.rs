pub fn lint_for_range_get_loop_end(from: f64, to: f64) -> f64 {
    from + (to - from).floor()
}
