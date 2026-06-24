use crate::functions::follow_type::follow_type_id;
use crate::functions::get_function_name_as_string::get_function_name_as_string;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::generic_type_pack_definition::GenericTypePackDefinition;
use crate::records::instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_fun::TypeFun;
use crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn instantiate_type_parameters(
        &mut self,
        scope: ScopePtr,
        base_type: TypeId,
        explicit_types: AstArray<AstTypeOrPack>,
        function_expr: *const AstExpr,
        location: &Location,
    ) -> TypeId {
        let base_type = unsafe { follow_type_id(base_type) };
        let function_type = unsafe { get_type_id::<FunctionType>(base_type).as_ref() };

        if let Some(ft) = function_type {
            let mut type_params = alloc::vec::Vec::with_capacity(ft.generics.len());
            for _ in &ft.generics {
                type_params.push(self.fresh_type_scope_ptr(scope.clone()));
            }
            let mut type_pack_params = alloc::vec::Vec::with_capacity(ft.generic_packs.len());
            for _ in &ft.generic_packs {
                type_pack_params.push(self.fresh_type_pack_scope_ptr(scope.clone()));
            }

            let mut type_param_count = 0;
            let mut type_pack_param_count = 0;
            let mut type_params_iter = type_params.iter_mut();
            let mut type_pack_params_iter = type_pack_params.iter_mut();

            for type_or_pack in explicit_types.iter() {
                if !type_or_pack.r#type.is_null() {
                    type_param_count += 1;
                    if let Some(param) = type_params_iter.next() {
                        *param = self.resolve_type(scope.clone(), unsafe { &*type_or_pack.r#type });
                    }
                } else {
                    type_pack_param_count += 1;
                    if let Some(param) = type_pack_params_iter.next() {
                        *param = self
                            .resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe {
                                &*type_or_pack.type_pack
                            });
                    }
                }
            }

            if type_param_count > ft.generics.len()
                || type_pack_param_count > ft.generic_packs.len()
            {
                let name = if !function_expr.is_null() {
                    get_function_name_as_string(unsafe { &*function_expr })
                } else {
                    None
                };
                self.report_error_location_type_error_data(location, crate::type_aliases::type_error_data::TypeErrorData::TypeInstantiationCountMismatch(crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch {
                    functionName: name,
                    functionType: base_type,
                    providedTypes: type_param_count,
                    maximumTypes: ft.generics.len(),
                    providedTypePacks: type_pack_param_count,
                    maximumTypePacks: ft.generic_packs.len(),
                }));
            }

            let mut base_fun = TypeFun::type_fun_type_id(base_type);
            base_fun.type_params.reserve(ft.generics.len());
            for &generic_id in &ft.generics {
                base_fun.type_params.push(GenericTypeDefinition {
                    ty: generic_id,
                    defaultValue: None,
                });
            }

            base_fun.type_pack_params.reserve(ft.generic_packs.len());
            for &generic_pack_id in &ft.generic_packs {
                base_fun.type_pack_params.push(GenericTypePackDefinition {
                    tp: generic_pack_id,
                    defaultValue: None,
                });
            }

            return self.instantiate_type_fun(
                &scope,
                &base_fun,
                &type_params,
                &type_pack_params,
                location,
            );
        }

        let mut edge_case = InstantiateGenericsOnNonFunction::None;
        if unsafe { !get_type_id::<IntersectionType>(base_type).is_null() } {
            edge_case = InstantiateGenericsOnNonFunction::Intersection;
        } else if let Some(mttv) = unsafe { get_type_id::<MetatableType>(base_type).as_ref() } {
            if self
                .get_index_type_from_type_impl(
                    scope,
                    mttv.metatable(),
                    &"__call".to_string(),
                    location,
                    false,
                )
                .is_some()
            {
                edge_case = InstantiateGenericsOnNonFunction::MetatableCall;
            }
        }

        self.report_error_location_type_error_data(
            location,
            crate::type_aliases::type_error_data::TypeErrorData::InstantiateGenericsOnNonFunction(
                InstantiateGenericsOnNonFunction {
                    interesting_edge_case: edge_case,
                },
            ),
        );
        base_type
    }
}
