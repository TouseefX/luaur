use luaur_vm::macros::getstr::getstr;

use crate::functions::try_find_local_name::try_find_local_name;
use crate::traits::LogAppend;

#[allow(non_snake_case)]
pub unsafe fn log_function_header(
    build: &mut dyn LogAppend,
    proto: *mut luaur_vm::records::proto::Proto,
) {
    let debugname = (*proto).debugname;
    if !debugname.is_null() {
        let name = getstr(debugname as *const _);
        let name_str = unsafe { core::ffi::CStr::from_ptr(name).to_string_lossy() };
        build.log_append(format_args!("; function {}(", name_str));
    } else {
        build.log_append(format_args!("; function("));
    }

    for i in 0..(*proto).numparams as i32 {
        let name = try_find_local_name(proto, i, 0);
        if !name.is_null() {
            let name_str = unsafe { core::ffi::CStr::from_ptr(name).to_string_lossy() };
            if i == 0 {
                build.log_append(format_args!("{}", name_str));
            } else {
                build.log_append(format_args!(", {}", name_str));
            }
        } else {
            if i == 0 {
                build.log_append(format_args!("$arg{}", i));
            } else {
                build.log_append(format_args!(", $arg{}", i));
            }
        }
    }

    if (*proto).numparams != 0 && (*proto).is_vararg != 0 {
        build.log_append(format_args!(", ...)"));
    } else {
        build.log_append(format_args!(")"));
    }

    if (*proto).linedefined >= 0 {
        build.log_append(format_args!(" line {}\n", (*proto).linedefined));
    } else {
        build.log_append(format_args!("\n"));
    }
}
