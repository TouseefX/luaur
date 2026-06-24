use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;
use luaur_common::FFlag;

pub fn is_shallow_inhabited(norm: &NormalizedType) -> bool {
    let luau_integer_type_2 = FFlag::LuauIntegerType2.get();

    let tops_is_never = unsafe { !get_type_id::<NeverType>(norm.tops).is_null() };
    let booleans_is_never = unsafe { !get_type_id::<NeverType>(norm.booleans).is_null() };
    let extern_types_is_never = !norm.extern_types.is_never();
    let errors_is_never = unsafe { !get_type_id::<NeverType>(norm.errors).is_null() };
    let nils_is_never = unsafe { !get_type_id::<NeverType>(norm.nils).is_null() };
    let numbers_is_never = unsafe { !get_type_id::<NeverType>(norm.numbers).is_null() };
    let strings_is_never = !norm.strings.is_never();
    let threads_is_never = unsafe { !get_type_id::<NeverType>(norm.threads).is_null() };
    let buffers_is_never = unsafe { get_type_id::<NeverType>(norm.buffers).is_null() };
    let functions_is_never = !norm.functions.is_never();
    let tables_not_empty = norm.tables.size() != 0;
    let tyvars_not_empty = !norm.tyvars.is_empty();

    if luau_integer_type_2 {
        let integers_is_never = unsafe { get_type_id::<NeverType>(norm.integers).is_null() };
        tops_is_never
            || booleans_is_never
            || extern_types_is_never
            || errors_is_never
            || nils_is_never
            || numbers_is_never
            || strings_is_never
            || threads_is_never
            || buffers_is_never
            || functions_is_never
            || tables_not_empty
            || tyvars_not_empty
            || integers_is_never
    } else {
        tops_is_never
            || booleans_is_never
            || extern_types_is_never
            || errors_is_never
            || nils_is_never
            || numbers_is_never
            || strings_is_never
            || threads_is_never
            || !unsafe { get_type_id::<NeverType>(norm.buffers).is_null() }
            || functions_is_never
            || tables_not_empty
            || tyvars_not_empty
    }
}
