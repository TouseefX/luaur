use crate::functions::emit_warning::emit_warning;
use crate::records::lint_redundant_native_attribute::LintRedundantNativeAttribute;
use luaur_ast::records::ast_attr::AstAttrType;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat::AstStat;
use luaur_config::enums::code::Code;

impl LintRedundantNativeAttribute {
    pub fn visit_ast_expr_function(&mut self, node: *mut AstExprFunction) -> bool {
        unsafe {
            let node = &*node;

            luaur_ast::visit::ast_stat_visit(node.body as *mut AstStat, self);

            for &attr in node.attributes.iter() {
                if !attr.is_null() && (*attr).r#type == AstAttrType::Native {
                    emit_warning(
                        &mut *self.context,
                        Code::Code_RedundantNativeAttribute,
                        (*attr).base.location,
                        format_args!(
                            "native attribute on a function is redundant in a native module; consider removing it"
                        ),
                    );
                }
            }
        }

        false
    }
}
