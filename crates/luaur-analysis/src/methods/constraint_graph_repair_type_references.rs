use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::dense_hash_set::DenseHashSet;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl ConstraintGraph {
    pub fn repair_type_references_type_id(&mut self, mut ty: TypeId) {
        let root = unsafe { follow_type_id(ty) };

        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());
        let _ = seen.insert(root);

        while !seen.contains(&ty) {
            let _ = seen.insert(ty);
            self.shift_references_type_id(ty, root);

            let bt_ptr = unsafe { crate::functions::get_type_alt_j::get_type_id::<BoundType>(ty) };
            if !bt_ptr.is_null() {
                let bt = unsafe { &*bt_ptr };
                ty = bt.boundTo;
            } else {
                break;
            }
        }
    }

    pub fn repair_type_references_type_pack_id(&mut self, mut ty: TypePackId) {
        let root = unsafe { follow_type_pack_id(ty) };

        let mut seen: DenseHashSet<TypePackId> = DenseHashSet::new(core::ptr::null());
        let _ = seen.insert(root);

        while !seen.contains(&ty) {
            let _ = seen.insert(ty);
            self.shift_references_type_pack_id(ty, root);

            let bt_ptr =
                unsafe { crate::functions::get_type_pack::get_type_pack_id::<BoundTypePack>(ty) };
            if !bt_ptr.is_null() {
                let bt = unsafe { &*bt_ptr };
                ty = bt.boundTo;
            } else {
                break;
            }
        }
    }
}
