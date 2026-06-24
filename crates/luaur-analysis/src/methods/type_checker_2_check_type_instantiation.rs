use crate::enums::interesting_edge_case::InterestingEdgeCase;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_identifier_of_base_var_type_checker_2::get_identifier_of_base_var_mut;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl TypeChecker2 {
    pub fn check_type_instantiation(
        &mut self,
        base_function_expr: *mut AstExpr,
        fn_type: TypeId,
        location: &Location,
        type_arguments: AstArray<AstTypeOrPack>,
    ) {
        LUAU_ASSERT!(FFlag::LuauExplicitTypeInstantiationSupport.get());

        let fn_type_followed = unsafe { follow_type_id(fn_type) };
        let ftv: *const FunctionType = unsafe { get_type_id::<FunctionType>(fn_type_followed) };

        if ftv.is_null() {
            let mut interesting_edge_case = InterestingEdgeCase::None;

            let has_call = unsafe {
                crate::functions::find_metatable_entry::find_metatable_entry(
                    self.builtin_types,
                    &mut (*self.module).errors,
                    fn_type,
                    "__call",
                    *location,
                )
            }
            .is_some();

            if has_call {
                interesting_edge_case = InterestingEdgeCase::MetatableCall;
            } else if !unsafe { get_type_id::<IntersectionType>(follow_type_id(fn_type)) }.is_null()
            {
                interesting_edge_case = InterestingEdgeCase::Intersection;
            }

            self.report_error_type_error_data_location(
                InstantiateGenericsOnNonFunction {
                    interesting_edge_case,
                }
                .into(),
                location,
            );
            return;
        }

        let mut type_count: usize = 0;
        let mut type_pack_count: usize = 0;

        for type_or_pack in type_arguments.iter() {
            if !type_or_pack.r#type.is_null() {
                type_count += 1;
            } else {
                LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
                type_pack_count += 1;
            }
        }

        let generics_len = unsafe { (*ftv).generics.len() };
        let generic_packs_len = unsafe { (*ftv).generic_packs.len() };

        if generics_len < type_count || generic_packs_len < type_pack_count {
            let function_name = get_identifier_of_base_var_mut(base_function_expr);
            self.report_error_type_error_data_location(
                TypeInstantiationCountMismatch {
                    functionName: function_name,
                    functionType: fn_type,
                    providedTypes: type_count,
                    maximumTypes: generics_len,
                    providedTypePacks: type_pack_count,
                    maximumTypePacks: generic_packs_len,
                }
                .into(),
                location,
            );
        }
    }
}
