use crate::type_aliases::log_luau_proc::LogLuauProc;

#[allow(non_upper_case_globals)]
pub(crate) static mut logLuau: LogLuauProc = None;

#[allow(non_snake_case)]
pub fn setLogLuau(ll: LogLuauProc) {
    unsafe {
        logLuau = ll;
    }
}
