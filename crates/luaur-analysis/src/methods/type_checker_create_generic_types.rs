use crate::records::duplicate_generic_parameter::DuplicateGenericParameter;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::generic_type_definitions::GenericTypeDefinitions;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::generic_type_pack_definition::GenericTypePackDefinition;
use crate::records::scope::Scope;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::type_fun::TypeFun;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_generic_type::AstGenericType;
use luaur_ast::records::ast_generic_type_pack::AstGenericTypePack;
use luaur_ast::records::ast_node::AstNode;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn create_generic_types(
        &mut self,
        scope: &ScopePtr,
        level_opt: Option<TypeLevel>,
        node: &AstNode,
        generic_names: &AstArray<*mut AstGenericType>,
        generic_pack_names: &AstArray<*mut AstGenericTypePack>,
        use_cache: bool,
    ) -> GenericTypeDefinitions {
        let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
        LUAU_ASSERT!(unsafe { (*scope_ptr).parent.is_some() });

        let level = level_opt.unwrap_or(unsafe { (*scope_ptr).level });

        let mut generics = alloc::vec::Vec::new();

        for generic in generic_names.iter() {
            let generic = unsafe { &**generic };
            let mut default_value = None;

            if !generic.default_value.is_null() {
                default_value =
                    Some(self.resolve_type(scope.clone(), unsafe { &*generic.default_value }));
            }

            let n = unsafe {
                core::ffi::CStr::from_ptr(generic.name.value)
                    .to_string_lossy()
                    .into_owned()
            };

            if unsafe {
                (*scope_ptr).private_type_bindings.contains_key(&n)
                    || (*scope_ptr).private_type_pack_bindings.contains_key(&n)
            } {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    node.location,
                    TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(
                        n.clone(),
                    )),
                ));
            }

            let g = if use_cache {
                let parent_ptr = unsafe {
                    (*scope_ptr).parent.as_ref().unwrap().as_ref() as *const Scope as *mut Scope
                };
                let existing = unsafe { (*parent_ptr).type_alias_type_parameters.get(&n).copied() };
                let cached = match existing {
                    Some(c) if !c.is_null() => c,
                    _ => {
                        let new_ty =
                            self.add_type(&GenericType::generic_type_type_level_name(level, &n));
                        unsafe {
                            (*parent_ptr)
                                .type_alias_type_parameters
                                .insert(n.clone(), new_ty);
                        }
                        new_ty
                    }
                };
                cached
            } else {
                self.add_type(&GenericType::generic_type_type_level_name(level, &n))
            };

            generics.push(GenericTypeDefinition {
                ty: g,
                defaultValue: default_value,
            });
            unsafe {
                (*scope_ptr)
                    .private_type_bindings
                    .insert(n, TypeFun::type_fun_type_id(g));
            }
        }

        let mut generic_packs = alloc::vec::Vec::new();

        for generic_pack in generic_pack_names.iter() {
            let generic_pack = unsafe { &**generic_pack };
            let mut default_value = None;

            if !generic_pack.default_value.is_null() {
                default_value = Some(
                    self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe {
                        &*generic_pack.default_value
                    }),
                );
            }

            let n = unsafe {
                core::ffi::CStr::from_ptr(generic_pack.name.value)
                    .to_string_lossy()
                    .into_owned()
            };

            if unsafe {
                (*scope_ptr).private_type_pack_bindings.contains_key(&n)
                    || (*scope_ptr).private_type_bindings.contains_key(&n)
            } {
                self.report_error_type_error(&TypeError::type_error_location_type_error_data(
                    node.location,
                    TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(
                        n.clone(),
                    )),
                ));
            }

            let parent_ptr = unsafe {
                (*scope_ptr).parent.as_ref().unwrap().as_ref() as *const Scope as *mut Scope
            };
            let existing = unsafe {
                (*parent_ptr)
                    .type_alias_type_pack_parameters
                    .get(&n)
                    .copied()
            };
            let cached = match existing {
                Some(c) if !c.is_null() => c,
                _ => {
                    let mut gtp = GenericTypePack {
                        index: 0,
                        level,
                        scope: core::ptr::null_mut(),
                        name: n.clone(),
                        explicitName: false,
                        polarity: crate::enums::polarity::Polarity::Unknown,
                    };
                    gtp.generic_type_pack_type_level_name(level, &n);
                    let new_tp = self.add_type_pack_type_pack_var(TypePackVar::from(gtp));
                    unsafe {
                        (*parent_ptr)
                            .type_alias_type_pack_parameters
                            .insert(n.clone(), new_tp);
                    }
                    new_tp
                }
            };

            generic_packs.push(GenericTypePackDefinition {
                tp: cached,
                defaultValue: default_value,
            });
            unsafe {
                (*scope_ptr).private_type_pack_bindings.insert(n, cached);
            }
        }

        GenericTypeDefinitions {
            generic_types: generics,
            generic_packs,
        }
    }
}
