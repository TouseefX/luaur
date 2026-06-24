use crate::records::lint_local_hygiene::LintLocalHygiene;
use crate::records::local_linter::Local;
use luaur_ast::records::ast_local::AstLocal;
use luaur_config::enums::code::Code;

impl LintLocalHygiene {
    pub fn report_unused_local(&mut self, local: *mut AstLocal, info: &Local) {
        let name = unsafe { (*local).name.value };
        if name.is_null() || unsafe { *name } == b'_' as core::ffi::c_char {
            return;
        }

        let (code, prefix) = if info.function {
            (Code::Code_FunctionUnused, "Function")
        } else if info.import {
            (Code::Code_ImportUnused, "Import")
        } else {
            (Code::Code_LocalUnused, "Variable")
        };

        crate::functions::emit_warning::emit_warning(
            unsafe { &mut *self.context },
            code,
            unsafe { (*local).location },
            format_args!(
                "{} '{}' is never used; prefix with '_' to silence",
                prefix,
                name_str(name)
            ),
        );
    }
}

fn name_str(value: *const core::ffi::c_char) -> alloc::borrow::Cow<'static, str> {
    if value.is_null() {
        alloc::borrow::Cow::Borrowed("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(value).to_string_lossy() }
    }
}
