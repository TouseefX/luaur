use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack::TypePack;
use crate::records::unifier::Unifier;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Unifier {
    pub fn occurs_check_dense_hash_set_type_pack_id_type_pack_id_type_pack_id(
        &mut self,
        seen: &mut DenseHashSet<TypePackId>,
        mut needle: TypePackId,
        mut haystack: TypePackId,
    ) -> bool {
        needle = self.log.follow_type_pack_id(needle);
        haystack = self.log.follow_type_pack_id(haystack);

        if seen.find(&haystack).is_some() {
            return false;
        }

        seen.insert(haystack);

        if !unsafe { get_type_pack_id::<ErrorTypePack>(needle) }.is_null() {
            return false;
        }

        if unsafe { get_type_pack_id::<FreeTypePack>(needle) }.is_null() {
            self.ice_string("Expected needle pack to be free");
        }

        while unsafe { get_type_pack_id::<ErrorTypePack>(haystack) }.is_null() {
            if needle == haystack {
                return true;
            }

            let pack = unsafe { get_type_pack_id::<TypePack>(haystack) };
            if !pack.is_null() {
                if let Some(tail) = unsafe { (*pack).tail } {
                    haystack = self.log.follow_type_pack_id(tail);
                    continue;
                }
            }

            break;
        }

        false
    }
}
