use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCacher {
    pub fn visit_type_pack_id_error_type_pack(
        &mut self,
        _tp: TypePackId,
        _etp: &ErrorTypePack,
    ) -> bool {
        true
    }
}
