use crate::functions::emit_warning::emit_warning;
use crate::records::lint_duplicate_local::LintDuplicateLocal;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_node::AstNode;
use luaur_config::enums::code::Code;

impl LintDuplicateLocal {
    pub fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
        unsafe {
            let node_ref = &*node;

            if !node_ref.self_.is_null() {
                *self.locals.get_or_insert(node_ref.self_) = node as *mut AstNode;
            }

            for i in 0..node_ref.args.size {
                let arg = *node_ref.args.data.add(i);
                *self.locals.get_or_insert(arg) = node as *mut AstNode;
            }

            for i in 0..node_ref.args.size {
                let local = *node_ref.args.data.add(i);
                let local_ref = &*local;

                if !local_ref.shadow.is_null()
                    && self.locals.find(&local_ref.shadow).copied() == Some(node as *mut AstNode)
                    && !self.ignore_duplicate(local)
                {
                    if local_ref.shadow == node_ref.self_ {
                        emit_warning(
                            &mut *self.context,
                            Code::Code_DuplicateLocal,
                            local_ref.location,
                            format_args!("Function parameter 'self' already defined implicitly"),
                        );
                    } else {
                        let shadow = &*local_ref.shadow;

                        if shadow.location.begin.line == local_ref.location.begin.line {
                            emit_warning(
                                &mut *self.context,
                                Code::Code_DuplicateLocal,
                                local_ref.location,
                                format_args!(
                                    "Function parameter '{}' already defined on column {}",
                                    name_str(local_ref.name.value),
                                    shadow.location.begin.column + 1
                                ),
                            );
                        } else {
                            emit_warning(
                                &mut *self.context,
                                Code::Code_DuplicateLocal,
                                local_ref.location,
                                format_args!(
                                    "Function parameter '{}' already defined on line {}",
                                    name_str(local_ref.name.value),
                                    shadow.location.begin.line + 1
                                ),
                            );
                        }
                    }
                }
            }
        }

        true
    }
}

fn name_str(value: *const core::ffi::c_char) -> alloc::borrow::Cow<'static, str> {
    if value.is_null() {
        alloc::borrow::Cow::Borrowed("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(value).to_string_lossy() }
    }
}
