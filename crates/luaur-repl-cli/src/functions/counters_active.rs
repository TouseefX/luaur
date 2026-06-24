use crate::functions::counters_init::G_COUNTERS;

// Faithful port of `bool countersActive() { return gCounters.L != nullptr; }`.
pub fn counters_active() -> bool {
    unsafe { !(*core::ptr::addr_of!(G_COUNTERS)).l.is_null() }
}
