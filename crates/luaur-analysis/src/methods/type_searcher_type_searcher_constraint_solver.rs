use crate::enums::polarity::Polarity;
use crate::records::type_searcher::TypeSearcher;
use crate::records::type_visitor::TypeVisitor;
use crate::type_aliases::type_id::TypeId;

impl TypeSearcher {
    pub fn type_searcher_type_id(needle: TypeId) -> Self {
        Self {
            base: TypeVisitor::new("TypeSearcher".to_string(), true),
            needle,
            current: Polarity::Positive,
            count: 0,
            result: Polarity::None,
        }
    }
}
