use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::r#type::Type;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct HasFreeType {
    pub base: TypeOnceVisitor,
    pub result: bool,
}

impl HasFreeType {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("TypeOnceVisitor"), true),
            result: false,
        }
    }

    pub fn has_free_type_has_free_type(&mut self) {
        *self = Self::new();
    }
}

impl GenericTypeVisitorTrait for HasFreeType {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        HasFreeType::visit_type_id(self, ty)
    }

    fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        HasFreeType::visit_type_pack_id(self, tp)
    }

    fn visit_type_id_extern_type(&mut self, ty: TypeId, ext: &ExternType) -> bool {
        self.visit_extern_type(ty, ext)
    }

    fn visit_type_id_free_type(&mut self, ty: TypeId, ft: &FreeType) -> bool {
        self.visit_free_type(ty, ft)
    }

    fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
        self.visit_free_type_pack(tp, ftp)
    }
}

impl HasFreeType {
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        if self.result || unsafe { (*ty).persistent } {
            return false;
        }
        true
    }

    pub fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        if self.result {
            return false;
        }
        true
    }

    pub fn visit_extern_type(&mut self, _ty: TypeId, _ext: &ExternType) -> bool {
        false
    }

    pub fn visit_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
        self.result = true;
        false
    }

    pub fn visit_free_type_pack(&mut self, _tp: TypePackId, _ftp: &FreeTypePack) -> bool {
        self.result = true;
        false
    }
}
