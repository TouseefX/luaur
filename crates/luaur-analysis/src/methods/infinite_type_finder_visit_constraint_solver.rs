use crate::records::infinite_type_finder::InfiniteTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl InfiniteTypeFinder {
    pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
        !self.found_infinite_type
    }
}
