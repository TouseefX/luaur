use crate::records::extern_type::ExternType;
use crate::records::free_type_searcher::FreeTypeSearcher;

use crate::type_aliases::type_id::TypeId;

impl FreeTypeSearcher {
    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
        false
    }
}
