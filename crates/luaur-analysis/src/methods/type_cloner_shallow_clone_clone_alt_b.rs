use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_cloner::TypeCloner;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCloner {
    pub fn shallow_clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let tp = unsafe { follow_type_pack_id(tp) };

        if let Some(clone) = self.find_type_pack_id(tp) {
            return clone;
        } else if unsafe { (*tp).persistent } && tp != self.force_tp {
            return tp;
        }

        let target = unsafe {
            (*self.arena).add_type_pack_type_pack_var(TypePackVar {
                ty: (*tp).ty.clone(),
                persistent: false,
                owningArena: core::ptr::null_mut(),
            })
        };

        // null for ordinary clones (Clone.cpp:194-197), fresh scope for the
        // fragment cloner override (Clone.cpp:531-534). Generic packs always null.
        unsafe {
            let generic = get_mutable_type_pack_id::<GenericTypePack>(target);
            if !generic.is_null() {
                (*generic).scope = core::ptr::null_mut();
            } else {
                let free = get_mutable_type_pack_id::<FreeTypePack>(target);
                if !free.is_null() {
                    (*free).scope = self.replacement_for_null_scope;
                }
            }
        }

        unsafe { (*self.packs).insert(tp, target) };
        self.queue.push(TypeOrPack::V1(target));
        target
    }
}
