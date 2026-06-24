use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::r#type::Type;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::unifiable::Error as ErrorType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

pub fn shallow_clone_type_id_type_arena_txn_log(
    ty: TypeId,
    dest: &mut TypeArena,
    log: *const TxnLog,
) -> TypeId {
    let mut ty = unsafe { (*log).follow_type_id(ty) };

    let pty = unsafe { (*log).pending_type_id(ty) };
    if !pty.is_null() {
        ty = unsafe { &(*pty).pending as *const Type };
    }

    let res_ty: TypeId = unsafe {
        match &(*ty).ty {
            // The pointer identities of free and local types is very important.
            // We decline to copy them.
            TypeVariant::Free(_a) => ty,
            TypeVariant::Bound(bound_to) => {
                // This should never happen, but visit() cannot see it.
                LUAU_ASSERT!(false);
                dest.add_tv(Type::new(TypeVariant::Bound(*bound_to)))
            }
            TypeVariant::Generic(a) => dest.add_type(a.clone()),
            TypeVariant::Blocked(a) => dest.add_type(a.clone()),
            TypeVariant::Primitive(_a) => {
                LUAU_ASSERT!((*ty).persistent);
                ty
            }
            TypeVariant::PendingExpansion(a) => {
                let clone = PendingExpansionType::pending_expansion_type_pending_expansion_type(
                    a.prefix,
                    a.name,
                    a.type_arguments.clone(),
                    a.pack_arguments.clone(),
                );
                dest.add_type(clone)
            }
            TypeVariant::Any(_a) => {
                LUAU_ASSERT!((*ty).persistent);
                ty
            }
            TypeVariant::NoRefine(_a) => {
                LUAU_ASSERT!((*ty).persistent);
                ty
            }
            TypeVariant::Error(a) => {
                LUAU_ASSERT!((*ty).persistent || a.synthetic.is_some());

                if (*ty).persistent {
                    ty
                } else {
                    // While this code intentionally works (and clones) even if `a.synthetic`
                    // is `std::nullopt`, we still assert above because we consider it a bug to
                    // have a non-persistent error type without any associated metadata. We
                    // should always use the persistent version in such cases.
                    let mut clone = ErrorType::new();
                    clone.synthetic = a.synthetic;
                    dest.add_tv(Type::new(TypeVariant::Error(clone)))
                }
            }
            TypeVariant::Unknown(_a) => {
                LUAU_ASSERT!((*ty).persistent);
                ty
            }
            TypeVariant::Never(_a) => {
                LUAU_ASSERT!((*ty).persistent);
                ty
            }
            TypeVariant::Lazy(_a) => ty,
            TypeVariant::Singleton(a) => dest.add_type(a.clone()),
            TypeVariant::Function(a) => {
                let clone = FunctionType {
                    level: a.level.clone(),
                    arg_types: a.arg_types,
                    ret_types: a.ret_types,
                    definition: a.definition.clone(),
                    has_self: a.has_self,
                    generics: a.generics.clone(),
                    generic_packs: a.generic_packs.clone(),
                    magic: a.magic.clone(),
                    tags: a.tags.clone(),
                    arg_names: a.arg_names.clone(),
                    is_checked_function: a.is_checked_function,
                    is_deprecated_function: a.is_deprecated_function,
                    deprecated_info: a.deprecated_info.clone(),
                    // Not copied by shallowClone; takes its default member initializer.
                    has_no_free_or_generic_types: false,
                };
                dest.add_type(clone)
            }
            TypeVariant::Table(a) => {
                LUAU_ASSERT!(a.bound_to.is_none());
                let mut clone = crate::records::table_type::TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                    &a.props,
                    a.indexer.clone(),
                    a.level.clone(),
                    a.scope,
                    a.state,
                );
                clone.definition_module_name = a.definition_module_name.clone();
                clone.definition_location = a.definition_location.clone();
                clone.name = a.name.clone();
                clone.synthetic_name = a.synthetic_name.clone();
                clone.instantiated_type_params = a.instantiated_type_params.clone();
                clone.instantiated_type_pack_params = a.instantiated_type_pack_params.clone();
                clone.tags = a.tags.clone();
                dest.add_type(clone)
            }
            TypeVariant::Metatable(a) => {
                let clone = MetatableType {
                    table: a.table,
                    metatable: a.metatable,
                    syntheticName: a.syntheticName.clone(),
                };
                dest.add_type(clone)
            }
            TypeVariant::Union(a) => {
                let clone = UnionType {
                    options: a.options.clone(),
                };
                dest.add_type(clone)
            }
            TypeVariant::Intersection(a) => {
                let clone = IntersectionType {
                    parts: a.parts.clone(),
                };
                dest.add_type(clone)
            }
            TypeVariant::Extern(a) => {
                let mut clone = ExternType {
                    name: a.name.clone(),
                    props: a.props.clone(),
                    parent: a.parent,
                    metatable: a.metatable,
                    tags: a.tags.clone(),
                    user_data: a.user_data.clone(),
                    definition_module_name: a.definition_module_name.clone(),
                    definition_location: a.definition_location.clone(),
                    indexer: a.indexer.clone(),
                    relation: None,
                };
                if FFlag::DebugLuauUserDefinedClasses.get() {
                    clone.relation = a.relation.clone();
                }
                dest.add_type(clone)
            }
            TypeVariant::Negation(a) => dest.add_type(NegationType { ty: a.ty }),
            TypeVariant::TypeFunctionInstance(a) => {
                let clone = TypeFunctionInstanceType::new(
                    a.function,
                    a.type_arguments.clone(),
                    a.pack_arguments.clone(),
                    a.user_func_name,
                    a.user_func_data.clone(),
                );
                dest.add_type(clone)
            }
        }
    };

    if res_ty != ty {
        unsafe {
            (*as_mutable_type_id(res_ty)).documentation_symbol = (*ty).documentation_symbol.clone();
        }
    }

    res_ty
}
