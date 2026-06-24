use luaur_vm::macros::getstr::getstr;
use luaur_vm::records::proto::Proto;

pub unsafe fn try_find_local_name(
    proto: *const Proto,
    reg: core::ffi::c_int,
    pcpos: core::ffi::c_int,
) -> *const core::ffi::c_char {
    for i in 0..(*proto).sizelocvars {
        let local = &*(*proto).locvars.add(i as usize);

        if reg == local.reg as core::ffi::c_int && pcpos >= local.startpc && pcpos < local.endpc {
            return if local.varname.is_null() {
                core::ptr::null()
            } else {
                getstr(local.varname)
            };
        }
    }

    core::ptr::null()
}
