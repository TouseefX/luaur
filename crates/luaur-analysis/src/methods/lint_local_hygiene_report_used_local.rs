use crate::records::lint_local_hygiene::LintLocalHygiene;
use crate::records::local_linter::Local;
use luaur_ast::records::ast_local::AstLocal;
use luaur_config::enums::code::Code;

impl LintLocalHygiene {
    pub fn report_used_local(&mut self, local: *mut AstLocal, info: &Local) {
        let shadow = unsafe { (*local).shadow };
        if !shadow.is_null() {
            let shadow_local = self.locals.find(&shadow);
            let duplicate_function_enabled = unsafe {
                (*self.context)
                    .options
                    .is_enabled(Code::Code_DuplicateFunction)
            };
            let duplicate_local_enabled = unsafe {
                (*self.context)
                    .options
                    .is_enabled(Code::Code_DuplicateLocal)
            };

            if duplicate_function_enabled
                && info.function
                && shadow_local.is_some_and(|shadow_info| shadow_info.function)
            {
                return;
            }

            if duplicate_local_enabled
                && shadow_local.is_some_and(|shadow_info| shadow_info.defined == info.defined)
            {
                return;
            }

            if unsafe { (*shadow).function_depth == (*local).function_depth } {
                crate::functions::emit_warning::emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_LocalShadow,
                    unsafe { (*local).location },
                    format_args!(
                        "Variable '{}' shadows previous declaration at line {}",
                        name_str(unsafe { (*local).name.value }),
                        unsafe { (*shadow).location.begin.line + 1 }
                    ),
                );
            }

            return;
        }

        if let Some(global) = self.globals.find(unsafe { &(*local).name }) {
            if global.builtin {
                return;
            }

            if !global.firstRef.is_null() {
                crate::functions::emit_warning::emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_LocalShadow,
                    unsafe { (*local).location },
                    format_args!(
                        "Variable '{}' shadows a global variable used at line {}",
                        name_str(unsafe { (*local).name.value }),
                        unsafe { (*global.firstRef).base.base.location.begin.line + 1 }
                    ),
                );
            } else {
                crate::functions::emit_warning::emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_LocalShadow,
                    unsafe { (*local).location },
                    format_args!(
                        "Variable '{}' shadows a global variable",
                        name_str(unsafe { (*local).name.value })
                    ),
                );
            }
        }
    }
}

fn name_str(value: *const core::ffi::c_char) -> alloc::borrow::Cow<'static, str> {
    if value.is_null() {
        alloc::borrow::Cow::Borrowed("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(value).to_string_lossy() }
    }
}
