use crate::records::indexer_index_collector::IndexerIndexCollector;
use crate::records::type_ids::TypeIds;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl IndexerIndexCollector {
    pub fn indexer_index_collector(indexes: *mut TypeIds) -> Self {
        IndexerIndexCollector {
            base: TypeOnceVisitor::new("IndexerIndexCollector".to_string(), true),
            indexes,
        }
    }
}
