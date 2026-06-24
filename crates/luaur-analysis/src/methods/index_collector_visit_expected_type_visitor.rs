//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ExpectedTypeVisitor.cpp:120:index_collector_visit`
//! Source: `Analysis/src/ExpectedTypeVisitor.cpp:109-148` (hand-ported)
//!
//! C++ `struct IndexCollector : TypeOnceVisitor` (anonymous namespace). The
//! virtual `visit(...)` overrides live as the `GenericTypeVisitorTrait` impl
//! (the `FindCyclicTypes`/`TypeFunctionFinder` precedents) so `traverse`
//! dispatches into them; the bodies delegate to the inherent methods declared
//! on the record (`index_collector.rs`) and on the sibling `visit_*` files.

use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::index_collector::IndexCollector;
use crate::records::intersection_type::IntersectionType;
use crate::records::table_type::TableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl GenericTypeVisitorTrait for IndexCollector {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    /// ExpectedTypeVisitor.cpp:120 — `bool visit(TypeId ty)`.
    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        IndexCollector::visit_type_id(self, ty)
    }

    /// ExpectedTypeVisitor.cpp:125 — `bool visit(TypeId, const UnionType&)`.
    fn visit_type_id_union_type(&mut self, ty: TypeId, utv: &UnionType) -> bool {
        IndexCollector::visit_type_id_union_type(self, ty, utv)
    }

    /// ExpectedTypeVisitor.cpp:130 — `bool visit(TypeId, const IntersectionType&)`.
    fn visit_type_id_intersection_type(&mut self, ty: TypeId, itv: &IntersectionType) -> bool {
        IndexCollector::visit_type_id_intersection_type(self, ty, itv)
    }

    /// ExpectedTypeVisitor.cpp:135 — `bool visit(TypeId, const TableType&)`.
    fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
        IndexCollector::visit_type_id_table_type(self, ty, ttv)
    }
}
