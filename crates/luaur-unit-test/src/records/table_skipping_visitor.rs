use alloc::string::String;
use alloc::vec::Vec;
use luaur_analysis::records::iterative_type_visitor::IterativeTypeVisitor;

#[derive(Debug, Clone)]
pub struct TableSkippingVisitor {
    pub(crate) base: IterativeTypeVisitor,
    pub(crate) trace: Vec<String>,
}
