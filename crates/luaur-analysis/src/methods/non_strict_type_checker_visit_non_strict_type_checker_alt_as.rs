use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_instantiate(
        &mut self,
        instantiate: *mut AstExprInstantiate,
    ) -> NonStrictContext {
        unsafe {
            let type_arguments = (*instantiate).type_arguments;
            for i in 0..type_arguments.size {
                let param: AstTypeOrPack = unsafe { *type_arguments.data.add(i) };
                if !param.r#type.is_null() {
                    self.visit_ast_type(param.r#type);
                } else {
                    self.visit_ast_type_pack(param.type_pack);
                }
            }

            self.visit_ast_expr_value_context((*instantiate).expr, ValueContext::RValue)
        }
    }
}
