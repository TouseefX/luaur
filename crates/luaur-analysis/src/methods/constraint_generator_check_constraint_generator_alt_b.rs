use crate::enums::polarity::Polarity;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::free_type::FreeType;
use crate::records::inference::Inference;
use crate::records::primitive_type_constraint::PrimitiveTypeConstraint;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_constant_string_optional_type_id_bool(
        &mut self,
        scope: &ScopePtr,
        string: *mut AstExprConstantString,
        expected_type: Option<TypeId>,
        force_singleton: bool,
    ) -> Inference {
        unsafe {
            let string_value = (*string).value;
            let string_data = string_value.data;
            let string_size = string_value.size;
            let string_bytes =
                core::slice::from_raw_parts(string_data as *const u8, string_size as usize);
            let string_str = core::str::from_utf8(string_bytes).unwrap_or("");
            let string_singleton = StringSingleton::new(String::from(string_str));
            let singleton_type = SingletonType::singleton_type(
                crate::type_aliases::singleton_variant::SingletonVariant::V1(string_singleton),
            );

            if force_singleton {
                return Inference::inference_type_id_refinement_id(
                    unsafe { (*self.arena).add_type(singleton_type) },
                    core::ptr::null_mut(),
                );
            }

            if self.large_table_depth > 0 {
                return Inference::inference_type_id_refinement_id(
                    (*self.builtin_types).stringType,
                    core::ptr::null_mut(),
                );
            }

            let free_ty = self.fresh_type(scope, Polarity::Positive);
            let ft = unsafe {
                crate::functions::get_mutable_type::get_mutable_type_id::<FreeType>(free_ty)
            };
            LUAU_ASSERT!(ft.is_null() == false);
            (*ft).lower_bound = unsafe { (*self.arena).add_type(singleton_type) };
            (*ft).upper_bound = (*self.builtin_types).stringType;

            self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                (*string).base.base.location,
                ConstraintV::PrimitiveType(PrimitiveTypeConstraint {
                    free_type: free_ty,
                    expected_type,
                    primitive_type: (*self.builtin_types).stringType,
                }),
            );
            Inference::inference_type_id_refinement_id(free_ty, core::ptr::null_mut())
        }
    }
}
