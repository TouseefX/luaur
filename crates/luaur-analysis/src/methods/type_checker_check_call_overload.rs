use crate::functions::get_error::get_type_error;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::count_mismatch::CountMismatch;
use crate::records::count_mismatch::CountMismatchContext;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::metatable_type::MetatableType;
use crate::records::never_type::NeverType;
use crate::records::overload_error_entry::OverloadErrorEntry;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::unifier_options::UnifierOptions;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn check_call_overload(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprCall,
        fn_ty: TypeId,
        ret_pack: TypePackId,
        arg_pack: TypePackId,
        args: &mut TypePack,
        arg_locations: &alloc::vec::Vec<Location>,
        arg_list_result: &WithPredicate<TypePackId>,
        overloads_that_match_arg_count: &mut Vec<TypeId>,
        overloads_that_dont: &mut Vec<TypeId>,
        errors: &mut Vec<OverloadErrorEntry>,
    ) -> Option<Box<WithPredicate<TypePackId>>> {
        let mut fn_ty =
            self.strip_from_nil_and_report(fn_ty, unsafe { &(*expr.func).base.location });

        if unsafe { !get_type_id::<AnyType>(fn_ty).is_null() } {
            self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
                self.any_type_pack,
                arg_pack,
                scope,
                &expr.base.base.location,
                CountMismatchContext::Arg,
            );
            return Some(Box::new(WithPredicate::with_predicate_t(
                self.any_type_pack,
            )));
        }

        if unsafe { !get_type_id::<ErrorType>(fn_ty).is_null() } {
            return Some(Box::new(WithPredicate::with_predicate_t(
                self.error_recovery_type_pack_scope_ptr(scope.clone()),
            )));
        }

        if unsafe { !get_type_id::<NeverType>(fn_ty).is_null() } {
            return Some(Box::new(WithPredicate::with_predicate_t(
                self.uninhabitable_type_pack,
            )));
        }

        if unsafe { !get_type_id::<FreeType>(fn_ty).is_null() } {
            // fn is one of the overloads of actualFunctionType, which
            // has been instantiated, so is a monotype. We can therefore
            // unify it with a monomorphic function.
            let mut function = FunctionType::function_type_new(arg_pack, ret_pack, None, false);
            function.level = scope.level;
            let r = self.add_type(&function);

            let mut options = UnifierOptions::default();
            options.is_function_call = true;
            self.unify_type_id_type_id_scope_ptr_location_unifier_options(
                r,
                fn_ty,
                scope,
                &expr.base.base.location,
                &options,
            );

            return Some(Box::new(WithPredicate::with_predicate_t(ret_pack)));
        }

        let mut meta_arg_locations: alloc::vec::Vec<Location> = alloc::vec::Vec::new();

        // Might be a callable table or class
        let mut call_ty: Option<TypeId> = None;
        let mttv = unsafe { get_type_id::<MetatableType>(fn_ty) };
        if !mttv.is_null() {
            call_ty = self.get_index_type_from_type(
                scope.clone(),
                unsafe { (*mttv).metatable },
                &Name::from("__call"),
                unsafe { &(*expr.func).base.location },
                false,
            );
        } else {
            let etv = unsafe { get_type_id::<ExternType>(fn_ty) };
            if !etv.is_null() {
                if let Some(metatable) = unsafe { (*etv).metatable } {
                    call_ty = self.get_index_type_from_type(
                        scope.clone(),
                        metatable,
                        &Name::from("__call"),
                        unsafe { &(*expr.func).base.location },
                        false,
                    );
                }
            }
        }

        let mut cur_arg_pack = arg_pack;
        let mut cur_args: *mut TypePack = args;
        let mut cur_arg_locations: alloc::vec::Vec<Location> = arg_locations.clone();

        if let Some(cty) = call_ty {
            // Construct arguments with 'self' added in front
            let meta_call_arg_pack = self.add_type_pack_type_pack(TypePack {
                head: args.head.clone(),
                tail: args.tail,
            });

            let meta_call_args =
                unsafe { get_mutable_type_pack_id::<TypePack>(meta_call_arg_pack) };
            unsafe { (*meta_call_args).head.insert(0, fn_ty) };

            meta_arg_locations = arg_locations.clone();
            meta_arg_locations.insert(0, unsafe { (*expr.func).base.location });

            fn_ty = self.instantiate(
                scope,
                cty,
                unsafe { (*expr.func).base.location },
                core::ptr::null(),
            );

            cur_arg_pack = meta_call_arg_pack;
            cur_args = meta_call_args;
            cur_arg_locations = meta_arg_locations.clone();
        }

        let ftv = unsafe { get_type_id::<FunctionType>(fn_ty) };
        if ftv.is_null() {
            self.report_error_location_type_error_data(
                unsafe { &(*expr.func).base.location },
                TypeErrorData::CannotCallNonFunction(CannotCallNonFunction { ty: fn_ty }),
            );
            let recovery = self.error_recovery_type_pack_scope_ptr(scope.clone());
            self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
                recovery,
                ret_pack,
                scope,
                unsafe { &(*expr.func).base.location },
                CountMismatchContext::FunctionResult,
            );
            return Some(Box::new(WithPredicate::with_predicate_t(
                self.error_recovery_type_pack_scope_ptr(scope.clone()),
            )));
        }

        // When this function type has magic functions and did return something, we select that overload instead.
        if let Some(magic) = unsafe { (*ftv).magic.clone() } {
            // TODO: We're passing in the wrong TypePackId. Should be argPack, but a unit test fails otherwise. CLI-40458
            if let Some(ret) = (magic.handle_old_solver)(self, scope, expr, arg_list_result.clone())
            {
                return Some(Box::new(WithPredicate::with_predicate_t_predicate_vec(
                    ret.r#type,
                    ret.predicates,
                )));
            }
        }

        let mut state = self.mk_unifier(scope, &expr.base.base.location);

        // Unify return typeArguments
        let ret_types = unsafe { (*ftv).ret_types };
        self.check_argument_list(
            scope,
            unsafe { &*expr.func },
            &mut state,
            ret_pack,
            ret_types,
            &alloc::vec::Vec::new(),
        );
        if !state.errors.is_empty() {
            return None;
        }

        let arg_types = unsafe { (*ftv).arg_types };
        self.check_argument_list(
            scope,
            unsafe { &*expr.func },
            &mut state,
            cur_arg_pack,
            arg_types,
            &cur_arg_locations,
        );

        if !state.errors.is_empty() {
            let mut arg_mismatch = false;
            for err in &state.errors {
                let cm = unsafe { get_type_error::<CountMismatch>(err) };
                if cm.is_null() {
                    continue;
                }

                if unsafe { (*cm).context } == CountMismatchContext::Arg {
                    arg_mismatch = true;
                    break;
                }
            }

            if !arg_mismatch {
                overloads_that_match_arg_count.push(fn_ty);
            } else {
                overloads_that_dont.push(fn_ty);
            }

            errors.push(OverloadErrorEntry {
                log: state.log.clone(),
                errors: state.errors.clone(),
                arguments: unsafe { (*cur_args).head.clone() },
                fn_ty: ftv,
            });
        } else {
            state.log.commit();

            unsafe {
                let module_ptr = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                    as *mut crate::records::module::Module;
                *(*module_ptr)
                    .ast_overload_resolved_types
                    .get_or_insert(expr as *const AstExprCall as *const AstNode) = fn_ty;
            }

            // We select this overload
            return Some(Box::new(WithPredicate::with_predicate_t(ret_pack)));
        }

        None
    }
}
