use crate::records::table_indexer::TableIndexer;
use crate::type_aliases::type_id::TypeId;

pub fn table_indexer_table_indexer(
    index_type: TypeId,
    index_result_type: TypeId,
    is_read_only: bool,
) -> TableIndexer {
    TableIndexer {
        index_type,
        index_result_type,
        is_read_only,
    }
}
