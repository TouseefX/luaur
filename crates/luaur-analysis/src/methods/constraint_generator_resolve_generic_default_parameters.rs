use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::emplace_type_pack::emplace_type_pack;
use crate::methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn resolve_generic_default_parameters(
        &mut self,
        defn_scope: *mut Scope,
        alias: *mut AstStatTypeAlias,
        fun: &TypeFun,
    ) {
        let alias_ref = unsafe { &*alias };
        LUAU_ASSERT!(alias_ref.generics.size == fun.type_params().len());

        for i in 0..alias_ref.generics.size {
            let ast_ty = unsafe { *alias_ref.generics.data.add(i) };
            let param = &fun.type_params()[i];

            if param.defaultValue.is_some() && !unsafe { (*ast_ty).default_value }.is_null() {
                let resolves_to = unsafe { (*ast_ty).default_value };
                let to_unblock = param.defaultValue.unwrap();
                let resolved = self.resolve_type(
                    defn_scope,
                    resolves_to,
                    /* in_type_arguments */ false,
                    /* replace_error_with_fresh */ false,
                    crate::enums::polarity::Polarity::Positive,
                );
                unsafe {
                    let mut resolved = resolved;
                    unifiable_bound_type_id_emplace_type_bound_type(
                        &mut *as_mutable_type_id(to_unblock),
                        &mut resolved,
                    );
                }
            }

            unsafe {
                let name_key = core::ffi::CStr::from_ptr((*ast_ty).name.value)
                    .to_string_lossy()
                    .into_owned();
                (*defn_scope)
                    .private_type_bindings
                    .insert(name_key, TypeFun::type_fun_type_id(param.ty));
            }
        }

        LUAU_ASSERT!(alias_ref.generic_packs.size == fun.type_pack_params().len());

        for i in 0..alias_ref.generic_packs.size {
            let ast_pack = unsafe { *alias_ref.generic_packs.data.add(i) };
            let param = &fun.type_pack_params()[i];

            if param.defaultValue.is_some() && !unsafe { (*ast_pack).default_value }.is_null() {
                let resolves_to = unsafe { (*ast_pack).default_value };
                let to_unblock = param.defaultValue.unwrap();
                let resolved = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
                    defn_scope,
                    resolves_to,
                    /* in_type_arguments */ false,
                    /* replace_error_with_fresh */ false,
                    crate::enums::polarity::Polarity::Positive,
                );
                unsafe {
                    emplace_type_pack(
                        crate::functions::as_mutable_type_pack::as_mutable_type_pack_id(to_unblock),
                        TypePackVariant::Bound(resolved),
                    );
                }
            }

            unsafe {
                let name_key = core::ffi::CStr::from_ptr((*ast_pack).name.value)
                    .to_string_lossy()
                    .into_owned();
                (*defn_scope)
                    .private_type_pack_bindings
                    .insert(name_key, param.tp);
            }
        }
    }
}
