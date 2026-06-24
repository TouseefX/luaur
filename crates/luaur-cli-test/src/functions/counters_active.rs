use crate::records::counters::gCounters;

pub fn counters_active() -> bool {
    unsafe { !gCounters.L.is_null() }
}
