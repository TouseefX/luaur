use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCacher {
    pub fn visit_type_pack_id_free_type_pack(
        &mut self,
        tp: TypePackId,
        _ftp: &FreeTypePack,
    ) -> bool {
        self.mark_uncacheable_type_pack_id(tp);
        false
    }
}
