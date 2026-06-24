//! C++ `TypeFunctionReductionResult<TypeId> indexFunctionImpl(
//! const std::vector<TypeId>& typeParams, const std::vector<TypePackId>&
//! packParams, NotNull<TypeFunctionContext> ctx, bool isRaw)`
//! (BuiltinTypeFunctions.cpp:2077-2209). Shared implementation behind `index`
//! and `rawget`.
//!
//! Vocabulary note: indexee refers to the type that contains the properties,
//! indexer refers to the type that is used to access indexee.
//! `index<Person, "name">` => `Person` is the indexee and `"name"` is the indexer.
use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_pending::is_pending;
use crate::functions::search_props_and_indexer::search_props_and_indexer;
use crate::functions::tbl_index_into_builtin_type_functions::tbl_index_into;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::extern_type::ExternType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

fn empty_location() -> Location {
    Location::new(
        Position { line: 0, column: 0 },
        Position { line: 0, column: 0 },
    )
}

/// C++ overload `bool tblIndexInto(TypeId indexer, TypeId indexee,
/// DenseHashSet<TypeId>& result, NotNull<TypeFunctionContext> ctx, bool isRaw)`
/// (BuiltinTypeFunctions.cpp:2068-2072): seeds an empty seen-set and delegates to
/// the recursive form.
fn tbl_index_into_2(
    indexer: TypeId,
    indexee: TypeId,
    result: &mut DenseHashSet<TypeId>,
    ctx: *mut TypeFunctionContext,
    is_raw: bool,
) -> bool {
    let mut seen_set: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());
    tbl_index_into(indexer, indexee, result, &mut seen_set, ctx, is_raw)
}

fn erroneous() -> TypeFunctionReductionResult {
    TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::Erroneous,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}

fn maybe_ok_blocked(blocked: Vec<TypeId>) -> TypeFunctionReductionResult {
    TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: blocked,
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}

fn is_boolean_singleton(ty: TypeId) -> bool {
    let singleton = unsafe { get_type_id::<SingletonType>(follow_type_id(ty)) };
    !singleton.is_null() && !get_singleton_type::<BooleanSingleton>(singleton).is_null()
}

fn is_nonempty_table(ty: TypeId) -> bool {
    let table = unsafe { get_type_id::<TableType>(follow_type_id(ty)) };
    unsafe { table.as_ref() }
        .map(|table| !table.props.is_empty())
        .unwrap_or(false)
}

pub fn index_function_impl(
    type_params: Vec<TypeId>,
    _pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
    is_raw: bool,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };

    let indexee_ty = unsafe { follow_type_id(type_params[0]) };

    if is_pending(indexee_ty, ctx_ref.solver) {
        return maybe_ok_blocked(vec![indexee_ty]);
    }

    let Some(indexee_norm_ty) =
        (unsafe { (*ctx_ref.normalizer.as_ptr()).try_normalize(indexee_ty) })
    else {
        return maybe_ok_blocked(vec![]);
    };

    // if the indexee is `any`, then indexing also gives us `any`.
    if indexee_norm_ty.should_suppress_errors() {
        return TypeFunctionReductionResult {
            result: Some(unsafe { ctx_ref.builtins.as_ref().anyType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    // if we don't have either just tables or just extern types, we've got nothing to index into
    if indexee_norm_ty.has_tables() == indexee_norm_ty.has_extern_types() {
        return erroneous();
    }

    // we're trying to reject any type that has not normalized to a table or
    // extern type or a union of tables or extern types.
    if indexee_norm_ty.has_tops()
        || indexee_norm_ty.has_booleans()
        || indexee_norm_ty.has_errors()
        || indexee_norm_ty.has_nils()
        || indexee_norm_ty.has_numbers()
        || indexee_norm_ty.has_strings()
        || indexee_norm_ty.has_threads()
        || indexee_norm_ty.has_buffers()
        || indexee_norm_ty.has_functions()
        || indexee_norm_ty.has_tyvars()
    {
        return erroneous();
    }

    let indexer_ty = unsafe { follow_type_id(type_params[1]) };

    if is_pending(indexer_ty, ctx_ref.solver) {
        return maybe_ok_blocked(vec![indexer_ty]);
    }

    let Some(indexer_norm_ty) =
        (unsafe { (*ctx_ref.normalizer.as_ptr()).try_normalize(indexer_ty) })
    else {
        return maybe_ok_blocked(vec![]);
    };

    // we're trying to reject any type that is not a string singleton or primitive
    if indexer_norm_ty.has_tops() || indexer_norm_ty.has_errors() {
        return erroneous();
    }

    // indexer can be a union —> break them down into a vector
    let single_type: Vec<TypeId> = vec![indexer_ty];
    let types_to_find: &Vec<TypeId> =
        if let Some(union_ty) = unsafe { get_type_id::<UnionType>(indexer_ty).as_ref() } {
            &union_ty.options
        } else {
            &single_type
        };

    let mut properties: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null()); // types that will be returned

    if indexee_norm_ty.has_extern_types() {
        LUAU_ASSERT!(!indexee_norm_ty.has_tables());

        if is_raw {
            // rawget should never reduce for extern types (to match the behavior
            // of the rawget global function)
            return erroneous();
        }

        // at least one class is guaranteed to be in the iterator by .hasExternTypes()
        for &extern_type_iter in &indexee_norm_ty.extern_types.ordering {
            let extern_ty = unsafe { get_type_id::<ExternType>(extern_type_iter) };
            if extern_ty.is_null() {
                LUAU_ASSERT!(false); // not possible according to normalization's spec
                return erroneous();
            }

            for &ty in types_to_find {
                // Search for all instances of indexer in class->props and class->indexer
                let extern_ref = unsafe { &*extern_ty };
                if search_props_and_indexer(
                    ty,
                    extern_ref.props.clone(),
                    extern_ref.indexer.clone(),
                    &mut properties,
                    ctx,
                ) {
                    continue; // found in this class, move to the next type
                }

                let mut parent = extern_ref.parent;
                let mut found_in_parent = false;
                while let Some(parent_ty) = parent {
                    if found_in_parent {
                        break;
                    }
                    let parent_extern_type =
                        unsafe { get_type_id::<ExternType>(follow_type_id(parent_ty)) };
                    let parent_ref = unsafe { &*parent_extern_type };
                    found_in_parent = search_props_and_indexer(
                        ty,
                        parent_ref.props.clone(),
                        parent_ref.indexer.clone(),
                        &mut properties,
                        ctx,
                    );
                    parent = parent_ref.parent;
                }

                // we move on to the next type if any of the parents had the property.
                if found_in_parent {
                    continue;
                }

                // property not found -> check in the metatable's __index.
                // findMetatableEntry demands the ability to emit errors, so we
                // must give it the state to do that, even if we eat the errors.
                let mut dummy: ErrorVec = vec![];
                let mm_type = unsafe {
                    crate::functions::find_metatable_entry::find_metatable_entry(
                        ctx_ref.builtins.as_ptr(),
                        &mut dummy,
                        extern_type_iter,
                        "__index",
                        empty_location(),
                    )
                };
                let mm_type = match mm_type {
                    Some(mm) => mm,
                    None => return erroneous(), // no metatable -> nowhere else to look
                };

                if !tbl_index_into_2(ty, mm_type, &mut properties, ctx, is_raw) {
                    // if indexer is not in the metatable, we fail to reduce
                    return erroneous();
                }
            }
        }
    }

    if indexee_norm_ty.has_tables() {
        LUAU_ASSERT!(!indexee_norm_ty.has_extern_types());

        // at least one table is guaranteed to be in the iterator by .hasTables()
        for &tables_iter in &indexee_norm_ty.tables.order {
            for &ty in types_to_find {
                if !tbl_index_into_2(ty, tables_iter, &mut properties, ctx, is_raw) {
                    if is_raw {
                        properties.insert(unsafe { ctx_ref.builtins.as_ref().nilType });
                    } else if is_boolean_singleton(ty) && is_nonempty_table(tables_iter) {
                        properties.insert(unsafe { ctx_ref.builtins.as_ref().unknownType });
                    } else {
                        return erroneous();
                    }
                }
            }
        }
    }

    // If the type being reduced to is a single type, no need to union
    if properties.size() == 1 {
        let only = *properties
            .iter()
            .next()
            .expect("properties has exactly one element");
        return TypeFunctionReductionResult {
            result: Some(only),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let options: Vec<TypeId> = properties.iter().copied().collect();
    let union_ty = unsafe { (*ctx_ref.arena.as_ptr()).add_type(UnionType { options }) };
    TypeFunctionReductionResult {
        result: Some(union_ty),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}
