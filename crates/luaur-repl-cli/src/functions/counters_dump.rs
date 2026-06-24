use core::ffi::{c_char, c_int, c_void, CStr};

use luaur_vm::functions::lua_getcounters::lua_getcounters;
use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::macros::lua_getref::lua_getref;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_debug::LuaDebug;

use crate::functions::counters_function_callback::counters_function_callback;
use crate::functions::counters_init::G_COUNTERS;
use crate::functions::counters_value_callback::counters_value_callback;
use crate::records::module_counters::ModuleCounters;

extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
}

// `extern "C"` adapters bridging the counter callbacks to lua_getcounters'
// `lua_CounterFunction` / `lua_CounterValue` function-pointer types.
unsafe extern "C" fn function_callback_cb(
    context: *mut c_void,
    function: *const c_char,
    line_defined: c_int,
) {
    counters_function_callback(context, function, line_defined);
}

unsafe extern "C" fn value_callback_cb(context: *mut c_void, kind: c_int, line: c_int, hits: u64) {
    counters_value_callback(context, kind, line, hits);
}

// Faithful port of `void countersDump(const char* path)`.
pub fn counters_dump(path: &str) {
    unsafe {
        let counters_ptr = core::ptr::addr_of_mut!(G_COUNTERS);
        let l = (*counters_ptr).l;

        let refs = (*counters_ptr).module_refs.clone();
        for fref in refs {
            lua_getref(l, fref);

            // C++ `lua_Debug ar = {}`.
            let mut ar: LuaDebug = core::mem::zeroed();
            lua_getinfo(l, -1, c"s".as_ptr(), &mut ar as *mut LuaDebug);

            (*counters_ptr).module_counters.push(ModuleCounters {
                name: if ar.short_src.is_null() {
                    alloc::string::String::new()
                } else {
                    CStr::from_ptr(ar.short_src).to_string_lossy().into_owned()
                },
                ..Default::default()
            });
            // Stable pointer to the just-pushed element; the callbacks only
            // mutate this element (never the outer vector), matching C++.
            let module_counters = (*counters_ptr).module_counters.last_mut().unwrap()
                as *mut ModuleCounters as *mut c_void;

            lua_getcounters(
                l,
                -1,
                module_counters,
                Some(function_callback_cb),
                Some(value_callback_cb),
            );

            lua_pop(l, 1);
        }

        let path_c = alloc::format!("{}\0", path);
        let f = fopen(path_c.as_ptr() as *const c_char, c"wb".as_ptr());
        if f.is_null() {
            eprintln!("Error opening counters file (callgrind) {}", path);
            return;
        }

        fputs(c"version: 1\n".as_ptr(), f);
        fputs(c"creator: Luau REPL\n".as_ptr(), f);
        fputs(c"events: Regular Fallback VmExit\n".as_ptr(), f);

        for module_counter in (*counters_ptr).module_counters.iter() {
            let name_c = alloc::format!("{}\0", module_counter.name);
            fprintf(f, c"fl=%s\n".as_ptr(), name_c.as_ptr());

            for function_counter in module_counter.functions.iter() {
                let fn_name_c = alloc::format!("{}\0", function_counter.name);
                fprintf(f, c"fn=%s\n".as_ptr(), fn_name_c.as_ptr());

                // BTreeMap already iterates by ascending line, matching the C++
                // "sorted by line" presentation requirement.
                for (line, counters) in function_counter.counters.iter() {
                    if counters.regularExecuted != 0
                        || counters.fallbackExecuted != 0
                        || counters.vmExitTaken != 0
                    {
                        fprintf(
                            f,
                            c"%d %lld %lld %lld\n".as_ptr(),
                            *line as c_int,
                            counters.regularExecuted as core::ffi::c_longlong,
                            counters.fallbackExecuted as core::ffi::c_longlong,
                            counters.vmExitTaken as core::ffi::c_longlong,
                        );
                    }
                }
            }
        }

        fclose(f);

        println!(
            "Counters data written to {} ({} modules)",
            path,
            (*counters_ptr).module_counters.len()
        );
    }
}
