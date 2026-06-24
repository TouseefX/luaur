//! Faithful port of `void profilerDump(const char* path)` (CLI/src/Profiler.cpp:115).

use alloc::collections::BTreeMap;
use core::ffi::{c_char, c_int, CStr};
use core::sync::atomic::Ordering;
use std::io::Write;

use luaur_vm::functions::lua_c_statename::luaC_statename;

use crate::functions::profiler_trigger::G_PROFILER;

pub fn profiler_dump(path: *const c_char) {
    unsafe {
        if path.is_null() {
            return;
        }
        let path_str = CStr::from_ptr(path).to_string_lossy().into_owned();
        let profiler = core::ptr::addr_of_mut!(G_PROFILER).as_mut().unwrap();

        let mut f = match std::fs::File::create(&path_str) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Error opening profile {}", path_str);
                return;
            }
        };

        let mut total: u64 = 0;
        let data = profiler.data.get_or_insert_with(BTreeMap::new);
        for (stack, &ticks) in data.iter() {
            // C++: fprintf(f, "%lld %s\n", ticks, stack)
            let _ = writeln!(f, "{} {}", ticks, stack);
            total += ticks;
        }
        drop(f);

        let stacks = data.len();
        let samples = profiler.samples.load(Ordering::Relaxed);
        println!(
            "Profiler dump written to {} (total runtime {:.3} seconds, {} samples, {} stacks)",
            path_str,
            total as f64 / 1e6,
            samples,
            stacks
        );

        let mut totalgc: u64 = 0;
        for &p in profiler.gc.iter() {
            totalgc += p;
        }

        if totalgc != 0 {
            print!(
                "GC: {:.3} seconds ({:.2}%)",
                totalgc as f64 / 1e6,
                totalgc as f64 / total as f64 * 100.0
            );

            for i in 0..profiler.gc.len() {
                let p = profiler.gc[i];
                if p != 0 {
                    let name_ptr = luaC_statename(i as c_int);
                    let name = if name_ptr.is_null() {
                        alloc::borrow::Cow::Borrowed("")
                    } else {
                        CStr::from_ptr(name_ptr).to_string_lossy()
                    };
                    print!(", {} {:.2}%", name, p as f64 / totalgc as f64 * 100.0);
                }
            }

            println!();
        }
    }
}
