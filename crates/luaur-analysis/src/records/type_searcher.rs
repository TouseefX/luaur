use crate::enums::polarity::Polarity;
use crate::records::type_visitor::TypeVisitor;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TypeSearcher {
    pub base: TypeVisitor,
    pub needle: TypeId,
    pub current: Polarity,
    pub count: usize,
    pub result: Polarity,
}
