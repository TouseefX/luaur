use crate::functions::are_equal_structural_type_equality_alt_e::are_equal_seen_set_type_item_type_item;
use crate::functions::are_seen::are_seen;
use crate::records::property_type::Property;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::type_aliases::seen_set_structural_type_equality::SeenSet;

#[allow(non_snake_case)]
pub fn are_equal_seen_set_table_type_table_type(
    seen: &mut SeenSet,
    lhs: &TableType,
    rhs: &TableType,
) -> bool {
    // are_seen expects BTreeSet<(*mut c_void, *mut c_void)>, but our SeenSet is BTreeSet<(*const c_void, *const c_void)>
    // We must transmute the mutability to match the expected signature.
    if are_seen(
        unsafe {
            core::mem::transmute::<
                &mut SeenSet,
                &mut std::collections::BTreeSet<(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
            >(seen)
        },
        lhs as *const TableType as *mut core::ffi::c_void,
        rhs as *const TableType as *mut core::ffi::c_void,
    ) {
        return true;
    }

    if lhs.state != rhs.state {
        return false;
    }

    if lhs.props.len() != rhs.props.len() {
        return false;
    }

    if (lhs.indexer.is_some()) != (rhs.indexer.is_some()) {
        return false;
    }

    if let (Some(l_indexer), Some(r_indexer)) = (&lhs.indexer, &rhs.indexer) {
        if l_indexer.is_read_only != r_indexer.is_read_only {
            return false;
        }

        if !are_equal_seen_set_type_item_type_item(
            seen,
            unsafe { &*l_indexer.index_type },
            unsafe { &*r_indexer.index_type },
        ) {
            return false;
        }

        if !are_equal_seen_set_type_item_type_item(
            seen,
            unsafe { &*l_indexer.index_result_type },
            unsafe { &*r_indexer.index_result_type },
        ) {
            return false;
        }
    }

    let mut l_iter = lhs.props.iter();
    let mut r_iter = rhs.props.iter();

    while let (Some((l_key, l_prop)), Some((r_key, r_prop))) = (l_iter.next(), r_iter.next()) {
        if l_key != r_key {
            return false;
        }

        if l_prop.read_ty.is_some() && r_prop.read_ty.is_some() {
            if !are_equal_seen_set_type_item_type_item(
                seen,
                unsafe { &**l_prop.read_ty.as_ref().unwrap() },
                unsafe { &**r_prop.read_ty.as_ref().unwrap() },
            ) {
                return false;
            }
        } else if l_prop.read_ty.is_some() || r_prop.read_ty.is_some() {
            return false;
        }

        if l_prop.write_ty.is_some() && r_prop.write_ty.is_some() {
            if !are_equal_seen_set_type_item_type_item(
                seen,
                unsafe { &**l_prop.write_ty.as_ref().unwrap() },
                unsafe { &**r_prop.write_ty.as_ref().unwrap() },
            ) {
                return false;
            }
        } else if l_prop.write_ty.is_some() || r_prop.write_ty.is_some() {
            return false;
        }
    }

    true
}
