use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::flatten_intersection::flatten_intersection;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn check_expr_pack_helper_scope_ptr_ast_expr_call(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprCall,
    ) -> WithPredicate<TypePackId> {
        // evaluate type of function
        // decompose an intersection into its component overloads
        // Compute typeArguments of parameters
        // For each overload
        //     Compare parameter and argument typeArguments
        //     Report any errors (also speculate dot vs colon warnings!)
        //     Return the resulting return type (even if there are errors)
        // If there are no matching overloads, unify with (a...) -> (b...) and return b...

        let mut self_type: TypeId = core::ptr::null_mut();
        let mut function_type: TypeId = core::ptr::null_mut();
        let actual_function_type: TypeId;

        if expr.self_ {
            let index_expr = unsafe {
                crate::rtti::ast_node_as::<AstExprIndexName>(
                    expr.func as *mut crate::records::ast_node::AstNode,
                )
            };
            if index_expr.is_null() {
                self.ice_string("method call expression has no 'self'");
            }

            self_type = self
                .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &*(*index_expr).expr },
                    None,
                    false,
                )
                .r#type;
            self_type =
                self.strip_from_nil_and_report(self_type, unsafe { &(*expr.func).base.location });

            let index_name: crate::type_aliases::name_type_infer::Name = unsafe {
                core::ffi::CStr::from_ptr((*index_expr).index.value)
                    .to_string_lossy()
                    .into_owned()
            };
            let prop_ty = self.get_index_type_from_type(
                scope.clone(),
                self_type,
                &index_name,
                &expr.base.base.location,
                /* addErrors= */ true,
            );
            if let Some(prop_ty) = prop_ty {
                function_type = prop_ty;
                let to_instantiate = if luaur_common::FFlag::LuauExplicitTypeInstantiationSupport
                    .get()
                    && expr.type_arguments.size != 0
                {
                    self.instantiate_type_parameters(
                        scope.clone(),
                        function_type,
                        expr.type_arguments,
                        expr.func as *const AstExpr,
                        &expr.base.base.location,
                    )
                } else {
                    function_type
                };
                actual_function_type = self.instantiate(
                    scope,
                    to_instantiate,
                    unsafe { (*expr.func).base.location },
                    core::ptr::null(),
                );
            } else {
                function_type = self.error_recovery_type_scope_ptr(scope);
                actual_function_type = function_type;
            }
        } else {
            function_type = self
                .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                    scope,
                    unsafe { &*expr.func },
                    None,
                    false,
                )
                .r#type;
            actual_function_type = self.instantiate(
                scope,
                function_type,
                unsafe { (*expr.func).base.location },
                core::ptr::null(),
            );
        }

        let ret_pack: TypePackId;
        let free = unsafe { get_type_id::<FreeType>(actual_function_type) };
        if !free.is_null() {
            ret_pack = self.fresh_type_pack_type_level(unsafe { (*free).level });
            let fresh_arg_pack = self.fresh_type_pack_type_level(unsafe { (*free).level });
            let level = unsafe { (*free).level };
            let mut function =
                FunctionType::function_type_new(fresh_arg_pack, ret_pack, None, false);
            function.level = level;
            unsafe {
                (*as_mutable_type_id(actual_function_type)).ty = TypeVariant::Function(function);
            }
        } else {
            ret_pack = self.fresh_type_pack_type_level(scope.level);
        }

        // We break this function up into a lambda here to limit our stack footprint.
        // The vectors used by this function aren't allocated until the lambda is actually called.

        // checkExpr will log the pre-instantiated type of the function.
        // That's not nearly as interesting as the instantiated type, which will include details about how
        // generic functions are being instantiated for this particular callsite.
        {
            let module_ptr = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            unsafe {
                *(*module_ptr)
                    .ast_original_call_types
                    .get_or_insert(expr.func as *const crate::records::ast_node::AstNode) =
                    follow_type_id(function_type);
                *(*module_ptr)
                    .ast_types
                    .get_or_insert(expr.func as *const AstExpr) = actual_function_type;
            }
        }

        let overloads = flatten_intersection(actual_function_type);

        let expected_types =
            self.get_expected_types_for_call(&overloads, expr.args.size, expr.self_);

        let arg_list_result = self.check_expr_list(
            scope,
            &expr.base.base.location,
            &expr.args,
            false,
            &alloc::vec::Vec::new(),
            &expected_types,
        );
        let mut arg_pack = arg_list_result.r#type;

        if unsafe { !get_type_pack_id::<ErrorTypePack>(arg_pack).is_null() } {
            return WithPredicate::with_predicate_t(
                self.error_recovery_type_pack_scope_ptr(scope.clone()),
            );
        }

        let mut arg_list_result = arg_list_result;
        if expr.self_ {
            arg_pack = self.add_type_pack_type_pack_var(TypePackVar::from(TypePack {
                head: alloc::vec::Vec::from([self_type]),
                tail: Some(arg_pack),
            }));
            arg_list_result.r#type = arg_pack;
        }
        let args = unsafe { get_mutable_type_pack_id::<TypePack>(arg_pack) };
        LUAU_ASSERT!(!args.is_null());

        let mut arg_locations: alloc::vec::Vec<luaur_ast::records::location::Location> =
            alloc::vec::Vec::new();
        arg_locations.reserve(expr.args.size + 1);
        if expr.self_ {
            let index_expr = unsafe {
                crate::rtti::ast_node_as::<AstExprIndexName>(
                    expr.func as *mut crate::records::ast_node::AstNode,
                )
            };
            arg_locations.push(unsafe { (*(*index_expr).expr).base.location });
        }
        for arg in expr.args.iter() {
            arg_locations.push(unsafe { (**arg).base.location });
        }

        let mut errors: alloc::vec::Vec<crate::records::overload_error_entry::OverloadErrorEntry> =
            alloc::vec::Vec::new(); // errors encountered for each overload

        let mut overloads_that_match_arg_count: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
        let mut overloads_that_dont: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();

        for fn_ty in overloads.iter() {
            let fn_ty = unsafe { follow_type_id(*fn_ty) };

            if let Some(ret) = self.check_call_overload(
                scope,
                expr,
                fn_ty,
                ret_pack,
                arg_pack,
                unsafe { &mut *args },
                &arg_locations,
                &arg_list_result,
                &mut overloads_that_match_arg_count,
                &mut overloads_that_dont,
                &mut errors,
            ) {
                return *ret;
            }
        }

        if self.handle_self_call_mismatch(
            scope,
            expr,
            unsafe { &mut *args },
            &arg_locations,
            &errors,
        ) {
            return WithPredicate::with_predicate_t(ret_pack);
        }

        self.report_overload_resolution_error(
            scope,
            expr,
            ret_pack,
            arg_pack,
            &arg_locations,
            &overloads,
            &overloads_that_match_arg_count,
            &mut errors,
        );

        let mut overload: *const FunctionType = core::ptr::null();
        if !overloads_that_match_arg_count.is_empty() {
            overload = unsafe { get_type_id::<FunctionType>(overloads_that_match_arg_count[0]) };
        }
        if overload.is_null() && !overloads_that_dont.is_empty() {
            overload = unsafe { get_type_id::<FunctionType>(overloads_that_dont[0]) };
        }
        if !overload.is_null() {
            return WithPredicate::with_predicate_t(
                self.error_recovery_type_pack_type_pack_id(unsafe { (*overload).ret_types }),
            );
        }

        WithPredicate::with_predicate_t(self.error_recovery_type_pack_type_pack_id(ret_pack))
    }
}
