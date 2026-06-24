use crate::functions::fast_is_subtype::fast_is_subtype;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn extract_matching_table_type_deprecated(
    tables: &mut Vec<TypeId>,
    expr_type: TypeId,
    builtin_types: *mut BuiltinTypes,
) -> Option<TypeId> {
    unsafe {
        LUAU_ASSERT!(!luaur_common::FFlag::LuauBidirectionalInferenceBetterUnionHandling.get());
        if tables.is_empty() {
            return None;
        }

        let expr_table = get_type_id::<TableType>(follow_type_id(expr_type));
        if expr_table.is_null() {
            return None;
        }
        let expr_table = &*expr_table;

        let mut table_count: usize = 0;
        let mut first_table: Option<TypeId> = None;

        for &ty in tables.iter() {
            let ty = follow_type_id(ty);
            let tt = get_type_id::<TableType>(ty);
            if !tt.is_null() {
                // If the expected table has a key whose type is a string or boolean
                // singleton and the corresponding exprType property does not match,
                // then skip this table.

                if first_table.is_none() {
                    first_table = Some(ty);
                }
                table_count += 1;

                for (name, expected_prop) in &(*tt).props {
                    let expected_read_ty = match expected_prop.read_ty {
                        Some(t) => t,
                        None => continue,
                    };

                    let expected_type = follow_type_id(expected_read_ty);

                    let st = get_type_id::<SingletonType>(expected_type);
                    if st.is_null() {
                        continue;
                    }

                    let it = match expr_table.props.get(name) {
                        Some(p) => p,
                        None => continue,
                    };

                    let expr_prop = it;

                    let expr_read_ty = match expr_prop.read_ty {
                        Some(t) => t,
                        None => continue,
                    };

                    let prop_type = follow_type_id(expr_read_ty);

                    let ft = get_type_id::<FreeType>(prop_type);

                    if !ft.is_null() && !get_type_id::<SingletonType>((*ft).lower_bound).is_null() {
                        if fast_is_subtype((*builtin_types).booleanType, (*ft).upper_bound)
                            && fast_is_subtype(expected_type, (*builtin_types).booleanType)
                        {
                            return Some(ty);
                        }

                        if fast_is_subtype((*builtin_types).stringType, (*ft).upper_bound)
                            && fast_is_subtype(expected_type, (*ft).lower_bound)
                        {
                            return Some(ty);
                        }
                    }

                    if fast_is_subtype(prop_type, expected_type) {
                        return Some(ty);
                    }
                }
            }
        }

        if table_count == 1 {
            LUAU_ASSERT!(first_table.is_some());
            return first_table;
        }

        None
    }
}
