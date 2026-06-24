use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_bounds::GenericBounds;
use crate::records::generic_type::GenericType;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;
use core::marker::PhantomData;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::{
    DenseDefault, DenseEq, DenseHashTable, DenseHasher, ItemInterface,
};

impl Default for GenericBounds {
    fn default() -> Self {
        GenericBounds {
            lower_bound: TypeIds::type_ids(),
            upper_bound: TypeIds::type_ids(),
        }
    }
}

impl DenseDefault for GenericBounds {
    fn dense_default() -> Self {
        GenericBounds::default()
    }
}

struct ItemInterfaceMapNoDefault<K, V>(PhantomData<(K, V)>);

impl<K: Clone, V> ItemInterface<K, (K, V)> for ItemInterfaceMapNoDefault<K, V> {
    fn get_key(item: &(K, V)) -> &K {
        &item.0
    }

    fn set_key(item: &mut (K, V), key: K) {
        item.0 = key;
    }

    fn make_empty(_empty_key: &K) -> (K, V) {
        unreachable!("find does not construct empty DenseHashMap entries")
    }
}

type DenseHashMapTable<K, V, H, E> =
    DenseHashTable<K, (K, V), ItemInterfaceMapNoDefault<K, V>, H, E>;

pub(crate) fn dense_hash_map_find_no_default<'a, K, V, H, E>(
    map: &'a DenseHashMap<K, V, H, E>,
    key: &K,
) -> Option<&'a V>
where
    K: Clone,
    H: DenseHasher<K> + Default,
    E: DenseEq<K> + Default,
{
    let table = unsafe {
        &*(map as *const DenseHashMap<K, V, H, E> as *const DenseHashMapTable<K, V, H, E>)
    };
    let item = luaur_common::methods::dense_hash_table_find::dense_hash_table_find(table, key);
    if item.is_null() {
        None
    } else {
        Some(unsafe { &(*item).1 })
    }
}

pub(crate) fn dense_hash_map_find_mut_no_default<'a, K, V, H, E>(
    map: &'a mut DenseHashMap<K, V, H, E>,
    key: &K,
) -> Option<&'a mut V>
where
    K: Clone,
    H: DenseHasher<K> + Default,
    E: DenseEq<K> + Default,
{
    let table = unsafe {
        &*(map as *const DenseHashMap<K, V, H, E> as *const DenseHashMapTable<K, V, H, E>)
    };
    let item = luaur_common::methods::dense_hash_table_find::dense_hash_table_find(table, key);
    if item.is_null() {
        None
    } else {
        Some(unsafe { &mut (*(item as *mut (K, V))).1 })
    }
}

impl Subtyping {
    pub fn bind_generic(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_ty: TypeId,
        super_ty: TypeId,
    ) -> bool {
        let sub_ty = unsafe { follow_type_id(sub_ty) };
        let super_ty = unsafe { follow_type_id(super_ty) };
        let mut original_sub_ty_bounds: Option<GenericBounds> = None;

        let super_bounds_snapshot = dense_hash_map_find_no_default(&env.mapped_generics, &super_ty)
            .and_then(|bounds| bounds.last().cloned());

        let sub_has_local_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &sub_ty)
            .map_or(false, |bounds| !bounds.is_empty());

        if sub_has_local_bounds {
            LUAU_ASSERT!(!unsafe { get_type_id::<GenericType>(sub_ty) }.is_null());

            let sub_bounds =
                dense_hash_map_find_mut_no_default(&mut env.mapped_generics, &sub_ty).unwrap();
            let sub_bounds_back = sub_bounds.last_mut().unwrap();
            original_sub_ty_bounds = Some(sub_bounds_back.clone());

            let upper_sub_bounds = &mut sub_bounds_back.upper_bound;

            if let Some(super_bounds) = &super_bounds_snapshot {
                LUAU_ASSERT!(!unsafe { get_type_id::<GenericType>(super_ty) }.is_null());

                self.maybe_update_bounds(
                    sub_ty,
                    super_ty,
                    upper_sub_bounds,
                    &super_bounds.lower_bound,
                    &super_bounds.upper_bound,
                );
            } else {
                upper_sub_bounds.insert_type_id(super_ty);
            }
        } else if env.contains_mapped_type(sub_ty) {
            unsafe {
                (*self.ice_reporter)
                    .ice_string("attempting to modify bounds of a potentially visited generic");
            }
        }

        let super_has_local_bounds =
            dense_hash_map_find_no_default(&env.mapped_generics, &super_ty)
                .map_or(false, |bounds| !bounds.is_empty());

        if super_has_local_bounds {
            LUAU_ASSERT!(!unsafe { get_type_id::<GenericType>(super_ty) }.is_null());

            let super_bounds =
                dense_hash_map_find_mut_no_default(&mut env.mapped_generics, &super_ty).unwrap();
            let super_bounds_back = super_bounds.last_mut().unwrap();
            let lower_super_bounds = &mut super_bounds_back.lower_bound;

            if let Some(original_sub_ty_bounds) = original_sub_ty_bounds {
                LUAU_ASSERT!(!unsafe { get_type_id::<GenericType>(sub_ty) }.is_null());

                self.maybe_update_bounds(
                    super_ty,
                    sub_ty,
                    lower_super_bounds,
                    &original_sub_ty_bounds.upper_bound,
                    &original_sub_ty_bounds.lower_bound,
                );
            } else {
                lower_super_bounds.insert_type_id(sub_ty);
            }
        } else if env.contains_mapped_type(super_ty) {
            unsafe {
                (*self.ice_reporter)
                    .ice_string("attempting to modify bounds of a potentially visited generic");
            }
        }

        true
    }
}
