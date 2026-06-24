use crate::enums::polarity::Polarity;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::scope::Scope;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_generic_type::AstGenericType;

impl ConstraintGenerator {
    pub fn create_generics(
        &mut self,
        scope: &ScopePtr,
        generics: AstArray<*mut AstGenericType>,
        use_cache: bool,
        add_types: bool,
    ) -> Vec<(Name, GenericTypeDefinition)> {
        let mut result: Vec<(Name, GenericTypeDefinition)> = Vec::new();

        let parent_scope = scope.parent.as_ref().unwrap();
        let parent_scope_ptr = parent_scope.as_ref() as *const Scope as *mut Scope;
        let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;

        for i in 0..generics.size {
            let generic = unsafe { *generics.data.add(i) };

            let generic_name_ptr = unsafe { (*generic).name.value };
            let generic_name = unsafe { core::ffi::CStr::from_ptr(generic_name_ptr) }
                .to_string_lossy()
                .into_owned();

            let generic_ty: TypeId;

            if use_cache {
                if let Some(ty) = parent_scope
                    .type_alias_type_parameters
                    .get(generic_name.as_str())
                {
                    generic_ty = *ty;
                } else {
                    generic_ty = unsafe {
                        (*self.arena).add_type(GenericType::generic_type_scope_name_polarity(
                            scope_ptr,
                            generic_name.clone(),
                            Polarity::None,
                        ))
                    };
                    unsafe {
                        (*parent_scope_ptr)
                            .type_alias_type_parameters
                            .insert(generic_name.clone(), generic_ty);
                    }
                }
            } else {
                generic_ty = unsafe {
                    (*self.arena).add_type(GenericType::generic_type_scope_name_polarity(
                        scope_ptr,
                        generic_name.clone(),
                        Polarity::None,
                    ))
                };
                unsafe {
                    (*parent_scope_ptr)
                        .type_alias_type_parameters
                        .insert(generic_name.clone(), generic_ty);
                }
            }

            let default_ty: Option<TypeId> = if unsafe { (*generic).default_value }.is_null() {
                None
            } else {
                unsafe { (*self.arena).add_type(BlockedType::default()) }.into()
            };

            if add_types {
                unsafe {
                    (*scope_ptr)
                        .private_type_bindings
                        .insert(generic_name.clone(), TypeFun::type_fun_type_id(generic_ty));
                }
            }

            result.push((
                generic_name,
                GenericTypeDefinition {
                    ty: generic_ty,
                    defaultValue: default_ty,
                },
            ));
        }

        result
    }
}
