//! C++ `LintUninitializedLocal::report` (`Analysis/src/Linter.cpp:2118`).
//!
//! The `LintUninitializedLocal` record carries a placeholder no-op `report`
//! method, so the faithful reporting logic lives here as a free function over
//! `&mut LintUninitializedLocal` and is invoked from `process`.

use crate::functions::emit_warning::emit_warning;
use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use luaur_config::enums::code::Code;

pub fn lint_uninitialized_local_report(pass: &mut LintUninitializedLocal) {
    let context = pass.context;

    for (local, l) in pass.locals.iter() {
        let local = *local;

        if l.defined && !l.initialized && !l.assigned && !l.first_use.is_null() {
            unsafe {
                let name = (*local).name.value;
                let name_str = if name.is_null() {
                    alloc::borrow::Cow::Borrowed("")
                } else {
                    core::ffi::CStr::from_ptr(name).to_string_lossy()
                };

                emit_warning(
                    &mut *context,
                    Code::Code_UninitializedLocal,
                    (*l.first_use).base.base.location,
                    format_args!(
                        "Variable '{}' defined at line {} is never initialized or assigned; initialize with 'nil' to silence",
                        name_str,
                        (*local).location.begin.line + 1
                    ),
                );
            }
        }
    }
}
