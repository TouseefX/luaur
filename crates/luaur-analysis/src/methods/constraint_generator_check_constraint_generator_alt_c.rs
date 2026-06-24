use crate::enums::polarity::Polarity;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::free_type::FreeType;
use crate::records::inference::Inference;
use crate::records::primitive_type_constraint::PrimitiveTypeConstraint;
use crate::records::singleton_type::SingletonType;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_constant_bool_optional_type_id_bool(
        &mut self,
        scope: &ScopePtr,
        bool_expr: *mut AstExprConstantBool,
        expected_type: Option<TypeId>,
        force_singleton: bool,
    ) -> Inference {
        unsafe {
            let singleton_type = if (*bool_expr).value {
                (*self.builtin_types).trueType
            } else {
                (*self.builtin_types).falseType
            };
            if force_singleton {
                return Inference::inference_type_id_refinement_id(
                    singleton_type,
                    core::ptr::null_mut(),
                );
            }

            if self.large_table_depth > 0 {
                return Inference::inference_type_id_refinement_id(
                    (*self.builtin_types).booleanType,
                    core::ptr::null_mut(),
                );
            }

            let free_ty = self.fresh_type(scope, Polarity::Positive);
            let ft = crate::functions::get_mutable_type::get_mutable_type_id::<FreeType>(free_ty);
            (*ft).lower_bound = singleton_type;
            (*ft).upper_bound = (*self.builtin_types).booleanType;

            self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                (*bool_expr).base.base.location,
                ConstraintV::PrimitiveType(PrimitiveTypeConstraint {
                    free_type: free_ty,
                    expected_type,
                    primitive_type: (*self.builtin_types).booleanType,
                }),
            );
            Inference::inference_type_id_refinement_id(free_ty, core::ptr::null_mut())
        }
    }
}
