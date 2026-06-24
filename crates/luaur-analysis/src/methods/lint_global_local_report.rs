use crate::functions::emit_warning::emit_warning;
use crate::records::lint_global_local::LintGlobalLocal;
use luaur_config::enums::code::Code;

impl LintGlobalLocal {
    pub fn report(&mut self) {
        let context = self.context;
        let placeholder = unsafe { (*context).placeholder };

        for i in 0..self.global_refs.len() {
            let gv = self.global_refs[i];
            let g = unsafe { self.globals.find(&(*gv).name) };

            match g {
                None => unsafe {
                    emit_warning(
                        &mut *context,
                        Code::Code_UnknownGlobal,
                        (*gv).base.base.location,
                        format_args!(
                            "Unknown global '{}'; consider assigning to it first",
                            name_str((*gv).name.value)
                        ),
                    );
                },
                Some(g) if !g.assigned && !g.builtin => unsafe {
                    emit_warning(
                        &mut *context,
                        Code::Code_UnknownGlobal,
                        (*gv).base.base.location,
                        format_args!(
                            "Unknown global '{}'; consider assigning to it first",
                            name_str((*gv).name.value)
                        ),
                    );
                },
                Some(g) => {
                    if let Some(replacement) = g.deprecated {
                        unsafe {
                            if !replacement.is_null()
                                && !core::ffi::CStr::from_ptr(replacement).to_bytes().is_empty()
                            {
                                emit_warning(
                                    &mut *context,
                                    Code::Code_DeprecatedGlobal,
                                    (*gv).base.base.location,
                                    format_args!(
                                        "Global '{}' is deprecated, use '{}' instead",
                                        name_str((*gv).name.value),
                                        name_str(replacement)
                                    ),
                                );
                            } else {
                                emit_warning(
                                    &mut *context,
                                    Code::Code_DeprecatedGlobal,
                                    (*gv).base.base.location,
                                    format_args!(
                                        "Global '{}' is deprecated",
                                        name_str((*gv).name.value)
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }

        for (_name, g) in self.globals.iter() {
            if !g.functionRef.is_empty()
                && g.assigned
                && unsafe { (*g.firstRef).name } != placeholder
            {
                let top = *g.functionRef.last().unwrap();

                unsafe {
                    if !(*top).debugname.value.is_null() {
                        emit_warning(
                            &mut *context,
                            Code::Code_GlobalUsedAsLocal,
                            (*g.firstRef).base.base.location,
                            format_args!(
                                "Global '{}' is only used in the enclosing function '{}'; consider changing it to local",
                                name_str((*g.firstRef).name.value),
                                name_str((*top).debugname.value)
                            ),
                        );
                    } else {
                        emit_warning(
                            &mut *context,
                            Code::Code_GlobalUsedAsLocal,
                            (*g.firstRef).base.base.location,
                            format_args!(
                                "Global '{}' is only used in the enclosing function defined at line {}; consider changing it to local",
                                name_str((*g.firstRef).name.value),
                                (*top).base.base.location.begin.line + 1
                            ),
                        );
                    }
                }
            } else if g.assigned
                && !g.readBeforeWritten
                && !g.definedInModuleScope
                && unsafe { (*g.firstRef).name } != placeholder
            {
                unsafe {
                    emit_warning(
                        &mut *context,
                        Code::Code_GlobalUsedAsLocal,
                        (*g.firstRef).base.base.location,
                        format_args!(
                            "Global '{}' is never read before being written. Consider changing it to local",
                            name_str((*g.firstRef).name.value)
                        ),
                    );
                }
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
