use crate::records::type_cacher::TypeCacher;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeCacher {
    pub fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        _tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        self.mark_uncacheable_type_pack_id(tp);
        false
    }
}
