use crate::records::indexer_index_collector::IndexerIndexCollector;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;

impl IndexerIndexCollector {
    pub fn visit_type_id_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
        true
    }
}
