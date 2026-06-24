use crate::enums::builtin_impl_type::BuiltinImplType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BuiltinImplResult {
    pub r#type: BuiltinImplType,
    pub actual_result_count: core::ffi::c_int,
}

impl Default for BuiltinImplResult {
    fn default() -> Self {
        Self {
            r#type: BuiltinImplType::None,
            actual_result_count: 0,
        }
    }
}
