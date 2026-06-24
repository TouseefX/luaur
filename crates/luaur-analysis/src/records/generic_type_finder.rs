use crate::enums::table_state::TableState;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::table_type::TableType;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct GenericTypeFinder {
    pub base: TypeOnceVisitor,
    pub found: bool,
}

impl GenericTypeFinder {
    pub fn generic_type_finder_generic_type_finder(&mut self) {
        self.found = false;
        self.base = TypeOnceVisitor::new(String::from("GenericTypeFinder"), true);
    }
}

impl GenericTypeFinder {
    pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
        !self.found
    }

    pub fn visit_type_pack_id(&mut self, _ty: TypePackId) -> bool {
        !self.found
    }

    pub fn visit_type_id_function_type(&mut self, _ty: TypeId, ftv: &FunctionType) -> bool {
        if ftv.has_no_free_or_generic_types {
            return false;
        }

        if !ftv.generics.is_empty() || !ftv.generic_packs.is_empty() {
            self.found = true;
        }

        !self.found
    }

    pub fn visit_type_id_table_type(&mut self, _ty: TypeId, ttv: &TableType) -> bool {
        // C++ `bool visit(TypeId, const Luau::TableType& ttv)` (Instantiation.h:122-128):
        // a generic table forces instantiation, so mark it found.
        if ttv.state == TableState::Generic {
            self.found = true;
        }

        !self.found
    }

    pub fn visit_type_id_generic_type(&mut self, _ty: TypeId, _gtv: &GenericType) -> bool {
        self.found = true;
        false
    }

    pub fn visit_type_pack_id_generic_type_pack(
        &mut self,
        _ty: TypePackId,
        _gtp: &GenericTypePack,
    ) -> bool {
        self.found = true;
        false
    }

    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _etv: &ExternType) -> bool {
        // During function instantiation, extern types are not traversed even if they have generics
        false
    }
}
