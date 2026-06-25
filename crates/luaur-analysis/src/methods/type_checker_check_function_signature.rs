use core::ffi::CStr;

use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::try_get_type_pack_type_at::try_get_type_pack_type_at;
use crate::records::ast_attr::AstAttrType;
use crate::records::binding::Binding;
use crate::records::function_argument::FunctionArgument;
use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn check_function_signature(
        &mut self,
        scope: &ScopePtr,
        sub_level: i32,
        expr: &AstExprFunction,
        original_name: Option<Location>,
        _self_type: Option<TypeId>,
        expected_type: Option<TypeId>,
    ) -> (TypeId, ScopePtr) {
        let fun_scope = self.child_function_scope(scope, &expr.base.base.location, sub_level);

        let mut expected_function_type: Option<&FunctionType> = None;
        if let Some(et) = expected_type {
            let et = unsafe { follow_type_id(et) };
            if let Some(ftv) = unsafe { get_type_id::<FunctionType>(et).as_ref() } {
                expected_function_type = Some(ftv);
            } else if let Some(utv) = unsafe { get_type_id::<UnionType>(et).as_ref() } {
                for option in &utv.options {
                    if let Some(ftv) =
                        unsafe { get_type_id::<FunctionType>(follow_type_id(*option)).as_ref() }
                    {
                        if expected_function_type.is_none() {
                            expected_function_type = Some(ftv);
                        } else {
                            expected_function_type = None;
                            break;
                        }
                    }
                }
            }
        }

        let generic_defs = self.create_generic_types(
            &fun_scope,
            None,
            &expr.base.base,
            &expr.generics,
            &expr.generic_packs,
            false,
        );

        let ret_pack = if let Some(ann) = unsafe { expr.return_annotation.as_ref() } {
            // if (expr.returnAnnotation) retPack = resolveTypePack(funScope, *expr.returnAnnotation);
            self.resolve_type_pack_scope_ptr_ast_type_pack(fun_scope.clone(), ann)
        } else if self.is_nonstrict_mode() {
            // else if (isNonstrictMode()) retPack = anyTypePack;
            self.any_type_pack
        } else if expected_function_type
            .map(|ftv| ftv.generics.is_empty() && ftv.generic_packs.is_empty())
            .unwrap_or(false)
        {
            // else if (expectedFunctionType && expectedFunctionType->generics.empty() && expectedFunctionType->genericPacks.empty())
            // auto [head, tail] = flatten(expectedFunctionType->retTypes);
            let (head, tail) = crate::functions::flatten_type_pack::flatten_type_pack_id(
                expected_function_type.unwrap().ret_types,
            );

            // Do not infer 'nil' as function return type
            // if (!tail && head.size() == 1 && isNil(head[0])) retPack = freshTypePack(funScope);
            // else retPack = addTypePack(head, tail);
            if tail.is_none() && head.len() == 1 && crate::functions::is_nil::is_nil(head[0]) {
                self.fresh_type_pack_scope_ptr(fun_scope.clone())
            } else {
                self.add_type_pack_vector_type_id_optional_type_pack_id(&head, tail)
            }
        } else {
            // else retPack = freshTypePack(funScope);
            self.fresh_type_pack_scope_ptr(fun_scope.clone())
        };

        unsafe {
            let fun_scope_mut =
                alloc::sync::Arc::as_ptr(&fun_scope) as *mut crate::records::scope::Scope;
            (*fun_scope_mut).return_type = ret_pack;
        }

        let mut vararg_pack: Option<TypePackId> = None;
        if expr.vararg {
            if !expr.vararg_annotation.is_null() {
                // funScope->varargPack = resolveTypePack(funScope, *expr.varargAnnotation);
                vararg_pack = Some(
                    self.resolve_type_pack_scope_ptr_ast_type_pack(fun_scope.clone(), unsafe {
                        &*expr.vararg_annotation
                    }),
                );
            } else {
                // if (expectedFunctionType && !isNonstrictMode())
                if let Some(expected) = expected_function_type {
                    if !self.is_nonstrict_mode() {
                        // auto [head, tail] = flatten(expectedFunctionType->argTypes);
                        let (mut head, tail) =
                            crate::functions::flatten_type_pack::flatten_type_pack_id(
                                expected.arg_types,
                            );

                        if expr.args.size <= head.len() {
                            // head.erase(head.begin(), head.begin() + expr.args.size);
                            head.drain(0..expr.args.size);
                            // funScope->varargPack = addTypePack(head, tail);
                            vararg_pack =
                                Some(self.add_type_pack_vector_type_id_optional_type_pack_id(
                                    &head, tail,
                                ));
                        } else if let Some(tail) = tail {
                            // if (get<VariadicTypePack>(follow(*tail)))
                            //     funScope->varargPack = addTypePack({}, tail);
                            let followed = unsafe {
                                crate::functions::follow_type_pack::follow_type_pack_id(tail)
                            };
                            if !unsafe {
                                crate::functions::get_type_pack::get_type_pack_id::<
                                    crate::records::variadic_type_pack::VariadicTypePack,
                                >(followed)
                            }
                            .is_null()
                            {
                                vararg_pack =
                                    Some(self.add_type_pack_vector_type_id_optional_type_pack_id(
                                        &alloc::vec::Vec::new(),
                                        Some(tail),
                                    ));
                            }
                        } else {
                            // funScope->varargPack = addTypePack({});
                            vararg_pack =
                                Some(self.add_type_pack_vector_type_id_optional_type_pack_id(
                                    &alloc::vec::Vec::new(),
                                    None,
                                ));
                        }
                    }
                }

                // TODO: should this be a free type pack? CLI-39910
                // if (!funScope->varargPack)
                //     funScope->varargPack = anyTypePack;
                if vararg_pack.is_none() {
                    vararg_pack = Some(self.any_type_pack);
                }
            }

            unsafe {
                let fun_scope_mut =
                    alloc::sync::Arc::as_ptr(&fun_scope) as *mut crate::records::scope::Scope;
                (*fun_scope_mut).vararg_pack = vararg_pack;
            }
        }

        let mut arg_types = Vec::new();
        if !expr.self_.is_null() {
            let fresh = self.fresh_type_scope_ptr(fun_scope.clone());
            let self_type = self.any_if_nonstrict(fresh);
            unsafe {
                let fun_scope_mut =
                    alloc::sync::Arc::as_ptr(&fun_scope) as *mut crate::records::scope::Scope;
                (*fun_scope_mut).bindings.insert(
                    Symbol::from_local(expr.self_),
                    Binding {
                        type_id: self_type,
                        location: (*expr.self_).location,
                        deprecated: false,
                        deprecated_suggestion: alloc::string::String::new(),
                        documentation_symbol: None,
                    },
                );
            }
            arg_types.push(self_type);
        }

        for (i, local) in expr.args.iter().enumerate() {
            let expected_arg_type =
                expected_function_type.and_then(|ftv| try_get_type_pack_type_at(ftv.arg_types, i));

            let arg_type = if let Some(ann) = unsafe { (**local).annotation.as_ref() } {
                let resolved = self.resolve_type(fun_scope.clone(), ann);
                if unsafe { !get_type_id::<ErrorType>(follow_type_id(resolved)).is_null() } {
                    if let Some(expected_arg_type) = expected_arg_type {
                        expected_arg_type
                    } else {
                        let fresh = self.fresh_type_scope_ptr(fun_scope.clone());
                        self.any_if_nonstrict(fresh)
                    }
                } else {
                    resolved
                }
            } else {
                if let Some(expected_arg_type) = expected_arg_type {
                    expected_arg_type
                } else {
                    let fresh = self.fresh_type_scope_ptr(fun_scope.clone());
                    self.any_if_nonstrict(fresh)
                }
            };
            unsafe {
                let fun_scope_mut =
                    alloc::sync::Arc::as_ptr(&fun_scope) as *mut crate::records::scope::Scope;
                (*fun_scope_mut).bindings.insert(
                    Symbol::from_local(*local),
                    Binding {
                        type_id: arg_type,
                        location: (**local).location,
                        deprecated: false,
                        deprecated_suggestion: alloc::string::String::new(),
                        documentation_symbol: None,
                    },
                );
            }
            arg_types.push(arg_type);
        }

        let arg_pack =
            self.add_type_pack_vector_type_id_optional_type_pack_id(&arg_types, vararg_pack);
        let defn = FunctionDefinition {
            definition_module_name: None,
            definition_location: expr.base.base.location,
            vararg_location: if expr.vararg {
                Some(expr.vararg_location)
            } else {
                None
            },
            original_name_location: original_name.unwrap_or(Location::new(
                expr.base.base.location.begin,
                expr.base.base.location.begin,
            )),
        };

        let mut function_type =
            FunctionType::function_type_new(arg_pack, ret_pack, Some(defn), !expr.self_.is_null());
        // C++ TypeInfer.cpp:4047-4058: if we have a generic expected function
        // type and no generics of our own, we should use the expected ones
        // (this is how an anonymous function passed where a generic function is
        // expected acquires the expected type's — possibly vestigial — generics,
        // e.g. `function(x: string) ... end` checked against `<a>(a) -> a`
        // becomes `<a>(string) -> string`).
        function_type.generics =
            if expected_function_type.is_some() && generic_defs.generic_types.is_empty() {
                expected_function_type.unwrap().generics.clone()
            } else {
                generic_defs
                    .generic_types
                    .iter()
                    .map(|def| def.ty)
                    .collect()
            };
        // C++ TypeInfer.cpp:4060-4071: likewise for generic type packs.
        function_type.generic_packs =
            if expected_function_type.is_some() && generic_defs.generic_packs.is_empty() {
                expected_function_type.unwrap().generic_packs.clone()
            } else {
                generic_defs
                    .generic_packs
                    .iter()
                    .map(|def| def.tp)
                    .collect()
            };
        function_type
            .arg_names
            .reserve(expr.args.len() + usize::from(!expr.self_.is_null()));
        if !expr.self_.is_null() {
            function_type.arg_names.push(Some(FunctionArgument {
                name: alloc::string::String::from("self"),
                location: Location::new(
                    expr.base.base.location.begin,
                    expr.base.base.location.begin,
                ),
            }));
        }
        for local in expr.args.iter() {
            function_type.arg_names.push(Some(FunctionArgument {
                name: unsafe { CStr::from_ptr((**local).name.value).to_string_lossy() }
                    .into_owned(),
                location: unsafe { (**local).location },
            }));
        }

        let fun_ty = self.add_type(&function_type);
        (fun_ty, fun_scope)
    }
}
