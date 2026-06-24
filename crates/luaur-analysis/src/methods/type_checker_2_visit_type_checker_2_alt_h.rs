use crate::functions::extend_type_pack::extend_type_pack;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_varargs::AstExprVarargs;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl TypeChecker2 {
    pub fn visit_ast_stat_return(&mut self, ret: *mut AstStatReturn) {
        unsafe {
            let location = (*ret).base.base.location;

            let scope_ptr = self.find_innermost_scope(location);
            let expected_ret_type = (*scope_ptr).return_type;

            let list = (*ret).list;
            if list.size == 0 {
                let empty_type_pack = (*self.builtin_types).emptyTypePack;
                self.test_is_subtype_type_pack_id_type_pack_id_location(
                    empty_type_pack,
                    expected_ret_type,
                    location,
                );
                return;
            }

            let builtin_types = self.builtin_types;
            // C++: extendTypePack(module->internalTypes, builtinTypes, expectedRetType, ret->list.size)
            let extended_pack = extend_type_pack(
                &mut (*self.module).internal_types,
                builtin_types,
                expected_ret_type,
                list.size as usize,
                Vec::new(),
            );

            let mut is_subtype = true;
            let mut actual_head: Vec<TypeId> = Vec::new();
            let mut actual_tail: Option<TypePackId> = None;

            let head = &extended_pack.head;
            let head_len = head.len();

            for idx in 0..(list.size as usize - 1) {
                let expr = *list.data.add(idx);
                if idx < head_len {
                    let expected_ty = head[idx];
                    let subtype_result =
                        self.test_literal_or_ast_type_is_subtype(expr, expected_ty);
                    is_subtype &= subtype_result;
                    actual_head.push(expected_ty);
                } else {
                    let ty = self.lookup_type(expr);
                    actual_head.push(ty);
                }
            }

            let last_idx = (list.size as usize) - 1;
            let last_expr = *list.data.add(last_idx);

            if head_len < list.size as usize
                || (*(last_expr as *mut luaur_ast::records::ast_node::AstNode)).is::<AstExprCall>()
                || (*(last_expr as *mut luaur_ast::records::ast_node::AstNode))
                    .is::<AstExprVarargs>()
            {
                actual_tail = Some(self.lookup_pack(last_expr));
            } else {
                let last_expected_ty = head[last_idx];
                let subtype_result =
                    self.test_literal_or_ast_type_is_subtype(last_expr, last_expected_ty);
                is_subtype &= subtype_result;
                actual_head.push(last_expected_ty);
            }

            if is_subtype {
                let reconstructed_ret_type =
                    (*self.module).internal_types.add_type_pack_t(TypePack {
                        head: actual_head,
                        tail: actual_tail,
                    });
                self.test_is_subtype_type_pack_id_type_pack_id_location(
                    reconstructed_ret_type,
                    expected_ret_type,
                    location,
                );
            }

            for i in 0..list.size as usize {
                let expr = *list.data.add(i);
                self.visit_ast_expr_value_context(
                    expr,
                    crate::enums::value_context::ValueContext::RValue,
                );
            }
        }
    }
}
