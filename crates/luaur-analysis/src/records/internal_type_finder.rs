use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct InternalTypeFinder {
    pub base: TypeOnceVisitor,
}

impl InternalTypeFinder {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("InternalTypeFinder"), true),
        }
    }

    pub fn visit_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
        false
    }

    pub fn visit_blocked_type(&mut self, _ty: TypeId, _bt: &BlockedType) -> bool {
        luaur_common::LUAU_ASSERT!(false);
        false
    }

    pub fn visit_free_type(&mut self, _ty: TypeId, _ft: &FreeType) -> bool {
        luaur_common::LUAU_ASSERT!(false);
        false
    }

    pub fn visit_pending_expansion_type(
        &mut self,
        _ty: TypeId,
        _pet: &PendingExpansionType,
    ) -> bool {
        luaur_common::LUAU_ASSERT!(false);
        false
    }

    pub fn visit_blocked_type_pack(&mut self, _tp: TypePackId, _btp: &BlockedTypePack) -> bool {
        luaur_common::LUAU_ASSERT!(false);
        false
    }

    pub fn visit_free_type_pack(&mut self, _tp: TypePackId, _ftp: &FreeTypePack) -> bool {
        luaur_common::LUAU_ASSERT!(false);
        false
    }

    pub fn visit_type_function_instance_type_pack(
        &mut self,
        _tp: TypePackId,
        _tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        luaur_common::LUAU_ASSERT!(false);
        false
    }
}
