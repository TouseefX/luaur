use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::apply_type_function::ApplyTypeFunction;
use crate::records::function_type::FunctionType;
use crate::records::module::Module;
use crate::records::recursive_restraint_violation::RecursiveRestraintViolation;
use crate::records::type_checker::TypeChecker;
use crate::records::type_fun::TypeFun;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::sync::Arc;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn instantiate_type_fun(
        &mut self,
        scope: &ScopePtr,
        tf: &TypeFun,
        type_params: &alloc::vec::Vec<TypeId>,
        type_pack_params: &alloc::vec::Vec<TypePackId>,
        location: &Location,
    ) -> TypeId {
        if tf.type_params.is_empty() && tf.type_pack_params.is_empty() {
            return tf.r#type;
        }

        let module_ptr =
            Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
        let arena = unsafe { &mut (*module_ptr).internal_types as *mut _ };
        let mut apply_type_function = ApplyTypeFunction::apply_type_function(arena);

        for (i, type_param) in tf.type_params.iter().enumerate() {
            if let Some(&argument) = type_params.get(i) {
                *apply_type_function
                    .type_arguments
                    .get_or_insert(type_param.ty) = argument;
            }
        }

        for (i, type_pack_param) in tf.type_pack_params.iter().enumerate() {
            if let Some(&argument) = type_pack_params.get(i) {
                *apply_type_function
                    .type_pack_arguments
                    .get_or_insert(type_pack_param.tp) = argument;
            }
        }

        let Some(instantiated) = apply_type_function.substitute_type_id(tf.r#type) else {
            self.report_error_location_type_error_data(
                location,
                TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
            );
            return self.error_recovery_type_scope_ptr(scope);
        };

        if apply_type_function.encountered_forwarded_type {
            self.report_error_location_type_error_data(
                location,
                TypeErrorData::RecursiveRestraintViolation(RecursiveRestraintViolation::default()),
            );
            return self.error_recovery_type_scope_ptr(scope);
        }

        let ftv = unsafe { get_mutable_type_id::<FunctionType>(instantiated) };
        if !ftv.is_null() {
            unsafe {
                (*ftv).generics.clear();
                (*ftv).generic_packs.clear();
            }
        }

        instantiated
    }
}
