use crate::enums::control_flow::ControlFlow;
use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::demoter::Demoter;
use crate::records::type_checker::TypeChecker;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl TypeChecker {
    pub fn check_scope_ptr_ast_stat_return(
        &mut self,
        scope: &ScopePtr,
        return_: &AstStatReturn,
    ) -> ControlFlow {
        let mut expected_types: alloc::vec::Vec<Option<TypeId>> = alloc::vec::Vec::new();
        expected_types.reserve(return_.list.size);

        let mut expected_ret_curr = begin(scope.return_type);
        let expected_ret_end = end(scope.return_type);

        for _i in 0..return_.list.size {
            if expected_ret_curr.operator_ne(&expected_ret_end) {
                expected_types.push(Some(*expected_ret_curr.operator_deref()));
                expected_ret_curr.operator_inc();
            } else if let Some(expected_args_tail) = expected_ret_curr.tail() {
                let vtp = unsafe {
                    get_type_pack_id::<VariadicTypePack>(follow_type_pack_id(expected_args_tail))
                };
                if !vtp.is_null() {
                    expected_types.push(Some(unsafe { (*vtp).ty }));
                }
            }
        }

        let arena = unsafe {
            &mut (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .internal_types as *mut crate::records::type_arena::TypeArena
        };
        let mut demoter = Demoter {
            arena,
            builtins: self.builtin_types,
        };
        demoter.demote(&mut expected_types);

        let ret_pack = self
            .check_expr_list(
                scope,
                &return_.base.base.location,
                &return_.list,
                false,
                &alloc::vec::Vec::new(),
                &expected_types,
            )
            .r#type;

        // HACK: Nonstrict mode gets a bit too smart and strict for us when we
        // start typechecking everything across module boundaries.
        let module_return_type = unsafe {
            (*self.current_module.as_ref().unwrap())
                .get_module_scope()
                .return_type
        };
        if self.is_nonstrict_mode()
            && unsafe {
                follow_type_pack_id(scope.return_type) == follow_type_pack_id(module_return_type)
            }
        {
            let errors = self.try_unify_type_pack_id_type_pack_id_scope_ptr_location(
                ret_pack,
                scope.return_type,
                scope.clone(),
                &return_.base.base.location,
            );

            if !errors.is_empty() {
                let any_pack = self.add_type_pack_initializer_list_type_id(&[self.any_type]);
                unsafe {
                    let module_scope = (*self.current_module.as_ref().unwrap()).get_module_scope();
                    let module_scope_mut = alloc::sync::Arc::as_ptr(&module_scope)
                        as *mut crate::records::scope::Scope;
                    (*module_scope_mut).return_type = any_pack;
                }
            }

            return ControlFlow::Returns;
        }

        self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
            ret_pack,
            scope.return_type,
            scope,
            &return_.base.base.location,
            CountMismatchContext::Return,
        );

        ControlFlow::Returns
    }
}
