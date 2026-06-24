use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::table_type::TableType;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct ContainsRefinableType {
    pub base: TypeOnceVisitor,
    pub found: bool,
}

impl ContainsRefinableType {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("ContainsRefinableType"), true),
            found: false,
        }
    }

    pub fn visit(&mut self, _ty: TypeId) -> bool {
        self.found = true;
        false
    }

    pub fn visit_no_refine(&mut self, _ty: TypeId, _nrt: &NoRefineType) -> bool {
        false
    }

    pub fn visit_table(&mut self, _ty: TypeId, _tt: &TableType) -> bool {
        !self.found
    }

    pub fn visit_metatable(&mut self, _ty: TypeId, _mt: &MetatableType) -> bool {
        !self.found
    }

    pub fn visit_function(&mut self, _ty: TypeId, _ft: &FunctionType) -> bool {
        !self.found
    }

    pub fn visit_union(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
        !self.found
    }

    pub fn visit_intersection(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
        !self.found
    }

    pub fn visit_negation(&mut self, _ty: TypeId, _nt: &NegationType) -> bool {
        !self.found
    }
}
