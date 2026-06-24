use crate::enums::control_flow::ControlFlow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::name_constraint::NameConstraint;
use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::reserved_identifier::ReservedIdentifier;
use crate::records::scope::Scope;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    // ConstraintGenerator::visit(const ScopePtr&, AstStatTypeAlias*)
    // (ConstraintGenerator.cpp).
    pub fn visit_scope_ptr_ast_stat_type_alias(
        &mut self,
        scope: &ScopePtr,
        alias: *mut AstStatTypeAlias,
    ) -> ControlFlow {
        let alias_ref = unsafe { &*alias };

        let alias_name = unsafe { core::ffi::CStr::from_ptr(alias_ref.name.value) };
        if alias_name.to_bytes() == b"%error-id%" {
            return ControlFlow::None;
        }

        if alias_name.to_bytes() == b"typeof" {
            self.report_error(
                alias_ref.base.base.location,
                TypeErrorData::ReservedIdentifier(ReservedIdentifier::new(String::from("typeof"))),
            );
            return ControlFlow::None;
        }

        let name_key: String = alias_name.to_string_lossy().into_owned();
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
        unsafe {
            (*scope_raw)
                .type_alias_locations
                .insert(name_key.clone(), alias_ref.base.base.location);
            (*scope_raw)
                .type_alias_name_locations
                .insert(name_key.clone(), alias_ref.name_location);
        }

        let defn_scope_opt = self
            .ast_type_alias_defining_scopes
            .find(&(alias as *const AstStatTypeAlias))
            .cloned();

        // These will be undefined if the alias was a duplicate definition, in which
        // case we just skip over it.
        let binding_it = unsafe {
            if alias_ref.exported {
                (*scope_raw).exported_type_bindings.get(&name_key).cloned()
            } else {
                (*scope_raw).private_type_bindings.get(&name_key).cloned()
            }
        };

        let (fun, defn_scope) = match (binding_it, defn_scope_opt) {
            (Some(b), Some(Some(s))) => (b, s),
            _ => return ControlFlow::None,
        };
        let defn_scope_raw = defn_scope.as_ref() as *const Scope as *mut Scope;

        self.resolve_generic_default_parameters(defn_scope_raw, alias, &fun);

        let ty = self.resolve_type(
            defn_scope_raw,
            alias_ref.type_ptr,
            /* in_type_arguments */ false,
            /* replace_error_with_fresh */ false,
            crate::enums::polarity::Polarity::Positive,
        );

        let alias_ty = fun.r#type();
        LUAU_ASSERT!(unsafe {
            !crate::functions::get_type_alt_j::get_type_id::<
                crate::records::blocked_type::BlockedType,
            >(alias_ty)
            .is_null()
        });

        if crate::functions::occurs_check_type_utils::occurs_check_type_id_type_id(alias_ty, ty) {
            unsafe {
                let mutable_alias = crate::functions::as_mutable_type::as_mutable_type_id(alias_ty);
                (*mutable_alias).ty = TypeVariant::Bound((*self.builtin_types).anyType);
            }
            self.report_error(
                alias_ref.name_location,
                TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
            );
        } else {
            unsafe {
                let mutable_alias = crate::functions::as_mutable_type::as_mutable_type_id(alias_ty);
                (*mutable_alias).ty = TypeVariant::Bound(ty);
            }
        }

        let type_params: Vec<TypeId> = self
            .create_generics(
                &defn_scope,
                alias_ref.generics,
                /* use_cache */ true,
                /* add_types */ false,
            )
            .into_iter()
            .map(|(_, def)| def.ty)
            .collect();

        let type_pack_params: Vec<TypePackId> = self
            .create_generic_packs(
                &defn_scope,
                alias_ref.generic_packs,
                /* use_cache */ true,
                /* add_types */ false,
            )
            .into_iter()
            .map(|(_, def)| def.tp)
            .collect();

        self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            unsafe { (*alias_ref.type_ptr).base.location },
            ConstraintV::Name(NameConstraint {
                named_type: ty,
                name: name_key,
                synthetic: false,
                type_parameters: type_params,
                type_pack_parameters: type_pack_params,
            }),
        );

        ControlFlow::None
    }
}
