use crate::records::collect_union_type_options::CollectUnionTypeOptions;
use crate::type_aliases::type_pack_id::TypePackId;

impl CollectUnionTypeOptions {
    pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        false
    }
}
