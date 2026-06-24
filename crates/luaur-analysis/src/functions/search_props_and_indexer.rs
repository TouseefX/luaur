//! C++ `bool searchPropsAndIndexer(TypeId ty, TableType::Props tblProps,
//! std::optional<TableIndexer> tblIndexer, DenseHashSet<TypeId>& result,
//! NotNull<TypeFunctionContext> ctx)` (BuiltinTypeFunctions.cpp:1913). The 2nd
//! parameter is `Props` (a `BTreeMap<Name, Property>`), shared by both
//! `TableType` and `ExternType` callers — NOT a whole `TableType`.
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_subtype_normalize_alt_b::is_subtype;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_indexer::TableIndexer;
use crate::records::type_function::TypeFunction;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn search_props_and_indexer(
    ty: TypeId,
    tbl_props: Props,
    tbl_indexer: Option<TableIndexer>,
    result: &mut DenseHashSet<TypeId>,
    ctx: *mut TypeFunctionContext,
) -> bool {
    let ctx_ref = unsafe { &*ctx };
    let ty = unsafe { follow_type_id(ty) };

    if let Some(singleton) = unsafe { get_type_id::<SingletonType>(ty).as_ref() } {
        let string_singleton = get_singleton_type::<StringSingleton>(singleton as *const _);
        if let Some(string_singleton) = unsafe { string_singleton.as_ref() } {
            if let Some(prop) = tbl_props.get(&string_singleton.value) {
                let Some(prop_ty) = prop.read_ty.or(prop.write_ty) else {
                    return false;
                };
                let prop_ty = unsafe { follow_type_id(prop_ty) };

                if let Some(prop_union_ty) = unsafe { get_type_id::<UnionType>(prop_ty).as_ref() } {
                    for &option in &prop_union_ty.options {
                        result.insert(unsafe { follow_type_id(option) });
                    }
                } else {
                    result.insert(prop_ty);
                }

                return true;
            }
        }
    }

    if let Some(tbl_indexer) = tbl_indexer {
        let mut index_type = unsafe { follow_type_id(tbl_indexer.index_type) };

        if let Some(tfit) = unsafe { get_type_id::<TypeFunctionInstanceType>(index_type).as_ref() }
        {
            let index_func: *const TypeFunction = &unsafe { ctx_ref.builtins.as_ref() }
                .typeFunctions
                .index_func;
            if core::ptr::eq(tfit.function.as_ptr() as *const TypeFunction, index_func) {
                index_type = unsafe { follow_type_id(tbl_indexer.index_result_type) };
            }
        }

        if is_subtype(
            ty,
            index_type,
            ctx_ref.arena.as_ptr(),
            ctx_ref.builtins.as_ptr(),
            ctx_ref.scope.as_ptr(),
            ctx_ref.normalizer.as_ptr(),
            ctx_ref.type_function_runtime.as_ptr(),
            ctx_ref.ice.as_ptr(),
        ) {
            let idx_result_ty = unsafe { follow_type_id(tbl_indexer.index_result_type) };

            if let Some(idx_res_union_ty) =
                unsafe { get_type_id::<UnionType>(idx_result_ty).as_ref() }
            {
                for &option in &idx_res_union_ty.options {
                    result.insert(unsafe { follow_type_id(option) });
                }
            } else {
                result.insert(idx_result_ty);
            }

            return true;
        }
    }

    false
}
