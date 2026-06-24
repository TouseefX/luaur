use crate::enums::polarity::Polarity;
use crate::records::type_searcher::TypeSearcher;
use crate::records::type_visitor::TypeVisitor;
use crate::type_aliases::type_id::TypeId;

impl TypeSearcher {
    pub fn type_searcher_type_id_polarity(needle: TypeId, initial_polarity: Polarity) -> Self {
        let mut result = Self::type_searcher_type_id(needle);
        result.current = initial_polarity;
        result
    }
}
