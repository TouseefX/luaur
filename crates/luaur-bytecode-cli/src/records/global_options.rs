#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalOptions {
    pub optimization_level: core::ffi::c_int,
    pub debug_level: core::ffi::c_int,
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            optimization_level: 1,
            debug_level: 1,
        }
    }
}

pub static mut globalOptions: GlobalOptions = GlobalOptions {
    optimization_level: 1,
    debug_level: 1,
};
