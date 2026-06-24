use crate::functions::allows_no_return_values::allows_no_return_values;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_end_location::get_end_location;
use crate::functions::get_fallthrough::get_fallthrough;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_exits_without_returning::FunctionExitsWithoutReturning;
use crate::records::function_type::FunctionType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat::AstStat;

impl TypeChecker {
    pub fn check_function_body(
        &mut self,
        scope: &ScopePtr,
        ty: TypeId,
        function: &AstExprFunction,
    ) {
        // LUAU_TIMETRACE_SCOPE("TypeChecker::checkFunctionBody", "TypeChecker");
        // (TimeTrace argument bookkeeping is a no-op in the Rust port.)

        let fun_ty = unsafe { get_mutable_type_id::<FunctionType>(ty) };
        if !fun_ty.is_null() {
            let fun_ty = unsafe { &mut *fun_ty };

            self.check_scope_ptr_ast_stat_block(scope, unsafe { &*function.body });

            // We explicitly don't follow here to check if we have a 'true' free type instead of bound one
            let ret_pack_is_bound =
                unsafe { matches!(&(*fun_ty.ret_types).ty, TypePackVariant::Bound(_)) };
            if !ret_pack_is_bound
                && unsafe { !get_type_pack_id::<FreeTypePack>(fun_ty.ret_types).is_null() }
            {
                let ret_pack = as_mutable_type_pack_id(fun_ty.ret_types);
                unsafe {
                    (*ret_pack).ty = TypePackVariant::TypePack(TypePack {
                        head: alloc::vec::Vec::new(),
                        tail: None,
                    });
                }
            }

            let reaches_implicit_return =
                !get_fallthrough(function.body as *const AstStat).is_null();

            if reaches_implicit_return
                && !allows_no_return_values(unsafe { follow_type_pack_id(fun_ty.ret_types) })
            {
                // If we're in nonstrict mode we want to only report this missing return
                // statement if there are type annotations on the function. In strict mode
                // we report it regardless.
                if !self.is_nonstrict_mode() || !function.return_annotation.is_null() {
                    self.report_error_location_type_error_data(
                        &get_end_location(function),
                        TypeErrorData::FunctionExitsWithoutReturning(
                            FunctionExitsWithoutReturning {
                                expected_return_type: fun_ty.ret_types,
                            },
                        ),
                    );
                }
            }

            let module_ptr = alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module;
            let key = function as *const AstExprFunction as *const AstExpr;
            if unsafe { (*module_ptr).ast_types.find(&key).is_none() } {
                unsafe {
                    *(*module_ptr).ast_types.get_or_insert(key) = ty;
                }
            }
        } else {
            self.ice_string("Checking non functional type");
        }
    }
}
