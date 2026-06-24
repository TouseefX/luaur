use crate::enums::control_flow::ControlFlow;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_assign(
        &mut self,
        scope: &ScopePtr,
        assign: *mut AstStatAssign,
    ) -> ControlFlow {
        let result_pack = self
            .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
                scope,
                unsafe { (*assign).values },
                &vec![],
            )
            .tp;
        let mut value_types = Vec::new();
        let (head, _) = flatten_type_pack_id(result_pack);

        unsafe {
            if head.len() >= (*assign).vars.size {
                for i in 0..(*assign).vars.size {
                    value_types.push(head[i]);
                }
            } else {
                for _ in 0..(*assign).vars.size {
                    value_types.push((*self.arena).add_type(BlockedType::default()));
                }
                let uc = self.add_constraint_scope_ptr_location_constraint_v(
                    scope,
                    (*assign).base.base.location,
                    ConstraintV::Unpack(UnpackConstraint {
                        result_pack: value_types.clone(),
                        source_pack: result_pack,
                    }),
                );
                for t in &value_types {
                    get_mutable_type_id::<BlockedType>(*t)
                        .as_mut()
                        .unwrap()
                        .set_owner(uc as *const _);
                }
            }
            for i in 0..(*assign).vars.size {
                self.visit_l_value_scope_ptr_ast_expr_type_id(
                    scope,
                    *(*assign).vars.data.add(i),
                    value_types[i],
                );
            }
        }
        ControlFlow::None
    }
}
