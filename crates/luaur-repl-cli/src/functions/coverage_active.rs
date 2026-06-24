use crate::functions::coverage_init::G_COVERAGE;

// Faithful port of `bool coverageActive() { return gCoverage.L != nullptr; }`.
pub fn coverage_active() -> bool {
    unsafe { !(*core::ptr::addr_of!(G_COVERAGE)).l.is_null() }
}
