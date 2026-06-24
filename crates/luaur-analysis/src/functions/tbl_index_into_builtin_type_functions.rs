//! C++ recursive `bool tblIndexInto(TypeId indexer, TypeId indexee,
//! DenseHashSet<TypeId>& result, DenseHashSet<TypeId>& seenSet,
//! NotNull<TypeFunctionContext> ctx, bool isRaw)`
//! (BuiltinTypeFunctions.cpp:1988-2066). Collects the types reachable by indexing
//! `indexee` with `indexer` into `result`.
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::search_props_and_indexer::search_props_and_indexer;
use crate::functions::solve_function_call::solve_function_call;
use crate::records::function_type::FunctionType;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use alloc::vec;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn tbl_index_into(
    indexer: TypeId,
    indexee: TypeId,
    result: &mut DenseHashSet<TypeId>,
    seen_set: &mut DenseHashSet<TypeId>,
    ctx: *mut TypeFunctionContext,
    is_raw: bool,
) -> bool {
    let ctx_ref = unsafe { &*ctx };

    let indexer = unsafe { follow_type_id(indexer) };
    let indexee = unsafe { follow_type_id(indexee) };

    if seen_set.contains(&indexee) {
        return false;
    }
    seen_set.insert(indexee);

    if let Some(union_ty) = unsafe { get_type_id::<UnionType>(indexee).as_ref() } {
        let mut res = true;
        for &component in &union_ty.options {
            // if the component is in the seen set and isn't the indexee itself,
            // we can skip it cause it means we encountered it in an earlier
            // component in the union.
            if seen_set.contains(&component) && component != indexee {
                continue;
            }
            res = res && tbl_index_into(indexer, component, result, seen_set, ctx, is_raw);
        }
        return res;
    }

    if !unsafe { get_type_id::<FunctionType>(indexee) }.is_null() {
        let arg_pack = unsafe {
            (*ctx_ref.arena.as_ptr()).add_type_pack_t(crate::records::type_pack::TypePack {
                head: vec![indexer],
                tail: None,
            })
        };

        let ret_pack = solve_function_call(
            ctx,
            unsafe { (*ctx_ref.scope.as_ptr()).location },
            indexee,
            arg_pack,
        );

        let ret_pack = match ret_pack {
            Some(rp) => rp,
            None => return false,
        };

        let extracted = extend_type_pack(
            unsafe { &mut *ctx_ref.arena.as_ptr() },
            ctx_ref.builtins.as_ptr(),
            ret_pack,
            1,
            Vec::new(),
        );
        if extracted.head.is_empty() {
            return false;
        }

        result.insert(unsafe { follow_type_id(extracted.head[0]) });
        return true;
    }

    // we have a table type to try indexing
    if let Some(table_ty) = unsafe { get_type_id::<TableType>(indexee).as_ref() } {
        return search_props_and_indexer(
            indexer,
            table_ty.props.clone(),
            table_ty.indexer.clone(),
            result,
            ctx,
        );
    }

    // we have a metatable type to try indexing
    if let Some(metatable_ty) = unsafe { get_type_id::<MetatableType>(indexee).as_ref() } {
        if let Some(table_ty) =
            unsafe { get_type_id::<TableType>(follow_type_id(metatable_ty.table())).as_ref() }
        {
            // try finding all properties within the current scope of the table
            if search_props_and_indexer(
                indexer,
                table_ty.props.clone(),
                table_ty.indexer.clone(),
                result,
                ctx,
            ) {
                return true;
            }
        }

        // if the code reached here, it means we weren't able to find all
        // properties -> look into __index metamethod
        if !is_raw {
            // findMetatableEntry demands the ability to emit errors, so we must
            // give it the necessary state to do that, even if we intend to just
            // eat the errors.
            let mut dummy: ErrorVec = vec![];
            let mm_type = unsafe {
                crate::functions::find_metatable_entry::find_metatable_entry(
                    ctx_ref.builtins.as_ptr(),
                    &mut dummy,
                    indexee,
                    "__index",
                    luaur_ast::records::location::Location::new(
                        luaur_ast::records::position::Position { line: 0, column: 0 },
                        luaur_ast::records::position::Position { line: 0, column: 0 },
                    ),
                )
            };
            if let Some(mm_type) = mm_type {
                return tbl_index_into(indexer, mm_type, result, seen_set, ctx, is_raw);
            }
        }
    }

    false
}
