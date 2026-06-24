use crate::enums::table_state::TableState;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::table_type::TableType;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use core::ffi::c_void;

#[derive(Debug, Clone)]
pub struct PromoteTypeLevels {
    pub base: TypeOnceVisitor,
    pub log: *mut TxnLog,
    pub type_arena: *const TypeArena,
    pub min_level: TypeLevel,
}

impl PromoteTypeLevels {
    pub fn new(log: &mut TxnLog, type_arena: &TypeArena, min_level: TypeLevel) -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("PromoteTypeLevels"), false),
            log: log as *mut TxnLog,
            type_arena,
            min_level,
        }
    }

    pub(crate) fn promote<T>(&mut self, ty: TypeId, t: *mut T, level: TypeLevel) {
        luaur_common::LUAU_ASSERT!(!t.is_null());
        if self.min_level.subsumes_strict(&level) {
            unsafe {
                if let Some(log) = self.log.as_mut() {
                    log.change_level_type_id_type_level(ty, self.min_level);
                }
            }
        }
    }

    pub(crate) fn promote_pack<T>(&mut self, tp: TypePackId, t: *mut T, level: TypeLevel) {
        luaur_common::LUAU_ASSERT!(!t.is_null());
        if self.min_level.subsumes_strict(&level) {
            unsafe {
                if let Some(log) = self.log.as_mut() {
                    log.change_level_type_pack_id_type_level(tp, self.min_level);
                }
            }
        }
    }

    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut TypeArena {
                return false;
            }
        }
        true
    }

    pub fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
        unsafe {
            let tp_var: *const TypePackVar = tp as *const TypePackVar;
            if (*tp_var).owningArena != self.type_arena as *mut TypeArena {
                return false;
            }
        }
        true
    }

    pub fn visit_free_type(&mut self, ty: TypeId, _ft: &FreeType) -> bool {
        unsafe {
            if !(*self.log).txn_log_is::<FreeType, TypeId>(ty) {
                return true;
            }
            let ft = (*self.log).txn_log_get_mutable::<FreeType, TypeId>(ty);
            self.promote(ty, ft, (*ft).level);
        }
        true
    }

    pub fn visit_function_type(&mut self, ty: TypeId, ft: &FunctionType) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut TypeArena {
                return false;
            }
            if !(*self.log).txn_log_is::<FunctionType, TypeId>(ty) {
                return true;
            }
            let ft_mut = (*self.log).txn_log_get_mutable::<FunctionType, TypeId>(ty);
            self.promote(ty, ft_mut, (*ft_mut).level);
        }
        true
    }

    pub fn visit_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
        unsafe {
            if (*ty).owning_arena != self.type_arena as *mut TypeArena {
                return false;
            }
            // Note: TableType state access requires specific field access or helper
            // Assuming TableType exposes state via a field or method
            // Placeholder: if ttv.state != TableState::Free && ttv.state != TableState::Generic { return true; }
            if !(*self.log).txn_log_is::<TableType, TypeId>(ty) {
                return true;
            }
            let ttv_mut = (*self.log).txn_log_get_mutable::<TableType, TypeId>(ty);
            // Assuming TableType has a level field
            // self.promote(ty, ttv_mut, (*ttv_mut).level);
        }
        true
    }

    pub fn visit_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
        unsafe {
            if !(*self.log).txn_log_is::<FreeTypePack, TypePackId>(tp) {
                return true;
            }
            let ftp = (*self.log).txn_log_get_mutable::<FreeTypePack, TypePackId>(tp);
            // Assuming FreeTypePack has a level field
            // self.promote_pack(tp, ftp, (*ftp).level);
        }
        true
    }
}
