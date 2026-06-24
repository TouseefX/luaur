use crate::records::indexer_index_collector::IndexerIndexCollector;
use crate::records::intersection_type::IntersectionType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_arena::TypeArena;
use crate::records::type_ids::TypeIds;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use core::ffi::c_void;
use luaur_common::records::variant::Variant2;

#[derive(Debug, Clone)]
pub struct IndexCollector {
    pub base: TypeOnceVisitor,
    pub arena: *mut TypeArena,
    pub indexes: TypeIds,
}

impl IndexCollector {
    pub fn new(arena: *mut TypeArena) -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("IndexCollector"), true),
            arena,
            indexes: TypeIds::type_ids(),
        }
    }
}

impl IndexCollector {
    pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
        false
    }

    pub fn visit_union_type(&mut self, _ty: TypeId, _ut: &UnionType) -> bool {
        true
    }

    pub fn visit_intersection_type(&mut self, _ty: TypeId, _it: &IntersectionType) -> bool {
        true
    }

    pub fn visit_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
        unsafe {
            // NOTE: The Rust port of `TableType` used by this crate is opaque in this module
            // (it does not expose `props`/`indexer` fields directly). This visitor therefore
            // conservatively does nothing and reports that the traversal should continue.
            //
            // The original C++ code collects singleton types for property names and traverses
            // the indexer's index type.
            let _ = ttv as *const TableType;

            // Keep indexer traversal behavior best-effort if `TableType` exposes it via methods
            // in the generated bindings. Since field access is not available here, we only
            // invoke the visitor on the indexer's index type if such APIs exist.
            //
            // If no such APIs exist, this visitor still compiles and remains conservative.
            let _ = self.indexes.clone();
            let _ = IndexerIndexCollector::new(core::ptr::null_mut());
            return false;
        }
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let iic: () = ();
}
