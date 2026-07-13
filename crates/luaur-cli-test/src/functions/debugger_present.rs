pub fn debugger_present() -> bool {
    #[cfg(windows)]
    {
        extern "system" {
            fn IsDebuggerPresent() -> core::ffi::c_int;
        }
        unsafe { IsDebuggerPresent() != 0 }
    }

    #[cfg(target_os = "macos")]
    {
        use core::mem;
        use core::ptr;

        #[repr(C)]
        struct kinfo_proc {
            kp_proc: extern_proc,
            // other fields omitted for brevity as they are not used
        }

        #[repr(C)]
        struct extern_proc {
            p_flag: core::ffi::c_int,
        }

        const CTL_KERN: core::ffi::c_int = 1;
        const KERN_PROC: core::ffi::c_int = 14;
        const KERN_PROC_PID: core::ffi::c_int = 1;
        const P_TRACED: core::ffi::c_int = 0x00000800;

        extern "C" {
            fn getpid() -> core::ffi::c_int;
            fn sysctl(
                name: *mut core::ffi::c_int,
                namelen: u32,
                oldp: *mut core::ffi::c_void,
                oldlenp: *mut usize,
                newp: *mut core::ffi::c_void,
                newlen: usize,
            ) -> core::ffi::c_int;
        }

        unsafe {
            let mut mib = [CTL_KERN, KERN_PROC, KERN_PROC_PID, getpid()];
            let mut info: kinfo_proc = mem::zeroed();
            let mut size = mem::size_of::<kinfo_proc>();

            let ret = sysctl(
                mib.as_mut_ptr(),
                mib.len() as u32,
                &mut info as *mut _ as *mut core::ffi::c_void,
                &mut size,
                ptr::null_mut(),
                0,
            );

            ret == 0 && (info.kp_proc.p_flag & P_TRACED) != 0
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = match File::open("/proc/self/status") {
            Ok(f) => f,
            Err(_) => return false,
        };

        let reader = BufReader::new(file);
        let mut tpid = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                if l.starts_with("TracerPid:\t") {
                    if let Some(val_str) = l.get(11..) {
                        tpid = val_str.trim().parse::<i32>().unwrap_or(0);
                    }
                    break;
                }
            }
        }

        tpid != 0
    }

    #[cfg(not(any(
        windows,
        target_os = "macos",
        target_os = "linux",
        target_os = "android"
    )))]
    {
        false
    }
}
