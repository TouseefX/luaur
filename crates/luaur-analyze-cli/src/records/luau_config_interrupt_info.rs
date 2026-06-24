use alloc::string::String;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct LuauConfigInterruptInfo {
    pub(crate) limits: TypeCheckLimits,
    pub(crate) module: String,
}
