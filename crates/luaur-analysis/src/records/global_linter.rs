use crate::type_aliases::type_id::TypeId;

/// C++ `LintContext::Global` (`Analysis/src/Linter.cpp:22`).
///
/// ```cpp
/// struct Global
/// {
///     TypeId type = nullptr;
///     std::optional<const char*> deprecated;
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Global {
    pub(crate) r#type: TypeId,
    pub(crate) deprecated: Option<*const core::ffi::c_char>,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            r#type: core::ptr::null(),
            deprecated: None,
        }
    }
}
