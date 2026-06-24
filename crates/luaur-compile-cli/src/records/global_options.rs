use core::ffi::c_char;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct GlobalOptions {
    pub optimizationLevel: i32,
    pub debugLevel: i32,
    pub typeInfoLevel: i32,

    pub vectorLib: *const c_char,
    pub vectorCtor: *const c_char,
    pub vectorType: *const c_char,

    pub onlyParse: bool,
    pub parseCst: bool,
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            optimizationLevel: 1,
            debugLevel: 1,
            typeInfoLevel: 0,
            vectorLib: core::ptr::null(),
            vectorCtor: core::ptr::null(),
            vectorType: core::ptr::null(),
            onlyParse: false,
            parseCst: false,
        }
    }
}

#[allow(non_upper_case_globals)]
pub static mut globalOptions: GlobalOptions = GlobalOptions {
    optimizationLevel: 1,
    debugLevel: 1,
    typeInfoLevel: 0,
    vectorLib: core::ptr::null(),
    vectorCtor: core::ptr::null(),
    vectorType: core::ptr::null(),
    onlyParse: false,
    parseCst: false,
};
