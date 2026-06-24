use crate::functions::default_log_luau::default_log_luau;
use crate::functions::set_log_luau::logLuau;

pub fn reset_log_luau_proc() {
    unsafe {
        logLuau = Some(default_log_luau);
    }
}
