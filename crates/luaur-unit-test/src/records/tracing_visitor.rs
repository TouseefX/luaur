use alloc::string::String;
use alloc::vec::Vec;
use luaur_analysis::records::iterative_type_visitor::IterativeTypeVisitor;
use luaur_analysis::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TracingVisitor {
    pub(crate) base: IterativeTypeVisitor,
    pub(crate) trace: Vec<String>,
    pub(crate) cycles: Vec<TypeId>,
}
