//! @interface-stub
use crate::records::index_collector::IndexCollector;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl IndexCollector {
    pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
        true
    }
}
