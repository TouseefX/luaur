use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::generic_error::GenericError;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_fun::TypeFun;
use crate::records::unknown_symbol::Context;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use luaur_common::functions::format::format;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn prototype_scope_ptr_ast_stat_declare_extern_type(
        &mut self,
        scope: ScopePtr,
        declared_extern_type: &AstStatDeclareExternType,
    ) {
        let mut super_ty: Option<TypeId> = Some(unsafe { (*self.builtin_types).externType });

        if let Some(super_name_astname) = declared_extern_type.super_name {
            let super_name: Name = unsafe {
                core::ffi::CStr::from_ptr(super_name_astname.value)
                    .to_string_lossy()
                    .into_owned()
            };
            let lookup_type = scope.lookup_type(&super_name);

            if lookup_type.is_none() {
                self.report_error_location_type_error_data(
                    &declared_extern_type.base.base.location,
                    TypeErrorData::UnknownSymbol(UnknownSymbol::new(super_name, Context::Type)),
                );
                self.incorrect_extern_type_definitions
                    .insert(declared_extern_type as *const AstStatDeclareExternType);
                return;
            }

            let lookup_type = lookup_type.unwrap();

            // We don't have generic extern typeArguments, so this assertion _should_ never be hit.
            LUAU_ASSERT!(
                lookup_type.type_params().is_empty() && lookup_type.type_pack_params().is_empty()
            );
            super_ty = Some(lookup_type.r#type());

            if unsafe { get_type_id::<ExternType>(follow_type_id(super_ty.unwrap())).is_null() } {
                let class_name = unsafe {
                    core::ffi::CStr::from_ptr(declared_extern_type.name.value).to_string_lossy()
                };
                self.report_error_location_type_error_data(
                    &declared_extern_type.base.base.location,
                    TypeErrorData::GenericError(GenericError::new(format(format_args!(
                        "Cannot use non-class type '{}' as a superclass of class '{}'",
                        unsafe {
                            core::ffi::CStr::from_ptr(super_name_astname.value).to_string_lossy()
                        },
                        class_name
                    )))),
                );
                self.incorrect_extern_type_definitions
                    .insert(declared_extern_type as *const AstStatDeclareExternType);
                return;
            }
        }

        let class_name: Name = unsafe {
            core::ffi::CStr::from_ptr(declared_extern_type.name.value)
                .to_string_lossy()
                .into_owned()
        };

        let module_name = unsafe { (*self.current_module.as_ref().unwrap()).name.clone() };

        let scope_level = scope.level;
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

        let class_ty: TypeId = unsafe {
            (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .internal_types
                .add_type(ExternType {
                    name: class_name.clone(),
                    props: Default::default(),
                    parent: super_ty,
                    metatable: None,
                    tags: Default::default(),
                    user_data: None,
                    definition_module_name: module_name,
                    definition_location: Some(declared_extern_type.base.base.location),
                    indexer: None,
                    relation: None,
                })
        };
        let etv = unsafe { get_mutable_type_id::<ExternType>(class_ty) };

        let meta_ty: TypeId = unsafe {
            (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .internal_types
                .add_type(TableType::table_type_table_state_type_level_scope(
                    TableState::Sealed,
                    scope_level,
                    scope_raw,
                ))
        };

        unsafe {
            (*etv).metatable = Some(meta_ty);
            (*scope_raw).exported_type_bindings.insert(
                class_name,
                TypeFun {
                    type_params: Default::default(),
                    type_pack_params: Default::default(),
                    r#type: class_ty,
                    definition_location: Some(declared_extern_type.base.base.location),
                },
            );
        }
    }
}
