use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference_pack::InferencePack;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;

impl ConstraintGenerator {
    pub fn check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
        &mut self,
        scope: &ScopePtr,
        exprs: AstArray<*mut AstExpr>,
        expected_types: &Vec<Option<TypeId>>,
    ) -> InferencePack {
        let mut head: Vec<TypeId> = Vec::new();
        let mut tail: Option<crate::type_aliases::type_pack_id::TypePackId> = None;

        for i in 0..exprs.size {
            let expr: *mut AstExpr = unsafe { *exprs.data.add(i) };
            if i < exprs.size - 1 {
                let mut expected_type: Option<TypeId> = None;
                if i < expected_types.len() {
                    expected_type = expected_types[i];
                }
                head.push(
                    self.check_scope_ptr_ast_expr_optional_type_id(scope, expr, expected_type)
                        .ty,
                );
            } else {
                let mut expected_tail_types: Vec<Option<TypeId>> = Vec::new();
                if i < expected_types.len() {
                    expected_tail_types.extend_from_slice(&expected_types[i..]);
                }
                tail = Some(
                    self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
                        scope,
                        expr,
                        &expected_tail_types,
                        true,
                    )
                    .tp,
                );
            }
        }

        InferencePack {
            tp: self.add_type_pack(head, tail),
            refinements: Vec::new(),
        }
    }
}
