use core::ffi::c_void;

use luaur_code_gen::enums::code_gen_counter::CodeGenCounter;

use crate::records::line_counters::LineCounters;
use crate::records::module_counters::ModuleCounters;

// Faithful port of Counters.cpp's `countersValueCallback`.
pub unsafe fn counters_value_callback(context: *mut c_void, kind: i32, line: i32, hits: u64) {
    let counters = &mut *(context as *mut ModuleCounters);
    let function = counters
        .functions
        .last_mut()
        .expect("countersValueCallback called before countersFunctionCallback");

    let entry = function
        .counters
        .entry(line)
        .or_insert_with(LineCounters::default);

    if kind == CodeGenCounter::RegularBlockExecuted as i32 {
        entry.regularExecuted += hits;
    } else if kind == CodeGenCounter::FallbackBlockExecuted as i32 {
        entry.fallbackExecuted += hits;
    } else if kind == CodeGenCounter::VmExitTaken as i32 {
        entry.vmExitTaken += hits;
    }
}
