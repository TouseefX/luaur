use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::size_type_pack::size;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::extra_information::ExtraInformation;
use crate::records::function_type::FunctionType;
use crate::records::generic_error::GenericError;
use crate::records::overload_error_entry::OverloadErrorEntry;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn report_overload_resolution_error(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprCall,
        ret_pack: TypePackId,
        arg_pack: TypePackId,
        arg_locations: &alloc::vec::Vec<Location>,
        overloads: &alloc::vec::Vec<TypeId>,
        overloads_that_match_arg_count: &alloc::vec::Vec<TypeId>,
        errors: &mut alloc::vec::Vec<OverloadErrorEntry>,
    ) {
        if overloads.len() == 1 {
            let error = errors
                .first_mut()
                .expect("single failed overload has errors");
            error.log.commit();
            let reported_errors = error.errors.clone();

            self.report_errors(&reported_errors);
            return;
        }

        let mut overload_types = overloads_that_match_arg_count.clone();
        if overloads_that_match_arg_count.is_empty() {
            self.report_error_location_type_error_data(
                &expr.base.base.location,
                TypeErrorData::GenericError(GenericError::new(format!(
                    "No overload for function accepts {} arguments.",
                    size(arg_pack, core::ptr::null_mut())
                ))),
            );

            overload_types = overloads.clone();
        } else {
            let overload = overloads_that_match_arg_count[0];
            overload_types.retain(|ty| *ty != overload);

            let ftv = unsafe { get_type_id::<FunctionType>(overload) };
            LUAU_ASSERT!(!ftv.is_null());

            let error_index = errors.iter().position(|e| e.fn_ty == ftv);
            LUAU_ASSERT!(error_index.is_some());

            let error = &mut errors[error_index.unwrap()];
            error.log.commit();
            let reported_errors = error.errors.clone();

            self.report_errors(&reported_errors);

            if overloads_that_match_arg_count.len() == 1 {
                return;
            }
        }

        let mut s = String::new();
        for (i, overload) in overload_types.iter().enumerate() {
            let overload = unsafe { follow_type_id(*overload) };
            let mut state = self.mk_unifier(scope, &expr.base.base.location);

            let ftv = unsafe { get_type_id::<FunctionType>(overload) };
            if !ftv.is_null() {
                self.check_argument_list(
                    scope,
                    unsafe { &*expr.func },
                    &mut state,
                    ret_pack,
                    unsafe { (*ftv).ret_types },
                    &alloc::vec::Vec::new(),
                );
                self.check_argument_list(
                    scope,
                    unsafe { &*expr.func },
                    &mut state,
                    arg_pack,
                    unsafe { (*ftv).arg_types },
                    arg_locations,
                );
            }

            if state.errors.is_empty() {
                state.log.commit();
            }

            if i > 0 {
                s.push_str("; ");
            }

            if i > 0 && i == overload_types.len() - 1 {
                s.push_str("and ");
            }

            s.push_str(&to_string_type_id(overload));
        }

        let message = if overloads_that_match_arg_count.is_empty() {
            String::from("Available overloads: ") + &s
        } else {
            String::from("Other overloads are also not viable: ") + &s
        };

        self.report_error_location_type_error_data(
            unsafe { &(*expr.func).base.location },
            TypeErrorData::ExtraInformation(ExtraInformation::new(message)),
        );
    }
}
