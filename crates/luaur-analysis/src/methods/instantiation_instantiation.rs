//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Instantiation.h:66:Instantiation`
//! Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:66-73, hand-ported)
use crate::records::builtin_types::BuiltinTypes;
use crate::records::instantiation::Instantiation;
use crate::records::replace_generics::ReplaceGenerics;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::tarjan::SubstitutionVtable;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ffi::c_void;

fn instantiation_is_dirty_ty(owner: *mut c_void, ty: TypeId) -> bool {
    unsafe { (*(owner as *mut Instantiation)).is_dirty_type_id(ty) }
}

fn instantiation_is_dirty_tp(_owner: *mut c_void, _tp: TypePackId) -> bool {
    false
}

fn instantiation_clean_ty(owner: *mut c_void, ty: TypeId) -> TypeId {
    unsafe { (*(owner as *mut Instantiation)).clean_type_id(ty) }
}

fn instantiation_clean_tp(_owner: *mut c_void, tp: TypePackId) -> TypePackId {
    tp
}

fn instantiation_found_dirty_ty(owner: *mut c_void, ty: TypeId) {
    unsafe {
        (*(owner as *mut Instantiation))
            .base
            .found_dirty_type_id(ty)
    }
}

fn instantiation_found_dirty_tp(owner: *mut c_void, tp: TypePackId) {
    unsafe {
        (*(owner as *mut Instantiation))
            .base
            .found_dirty_type_pack_id(tp)
    }
}

fn instantiation_ignore_children_ty(owner: *mut c_void, ty: TypeId) -> bool {
    unsafe { (*(owner as *mut Instantiation)).ignore_children(ty) }
}

fn instantiation_ignore_children_tp(_owner: *mut c_void, _tp: TypePackId) -> bool {
    false
}

impl Instantiation {
    /// C++ `Instantiation(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
    /// TypeLevel level, Scope* scope) : Substitution(log, arena), builtinTypes(builtinTypes),
    /// level(level), scope(scope), reusableReplaceGenerics(log, arena, builtinTypes, level, scope, {}, {})`.
    pub fn instantiation_new(
        log: *const TxnLog,
        arena: *mut TypeArena,
        builtin_types: *mut BuiltinTypes,
        level: TypeLevel,
        scope: *mut Scope,
    ) -> Self {
        Instantiation {
            base: Substitution::substitution_new(log, arena),
            builtin_types,
            level,
            scope,
            reusable_replace_generics: ReplaceGenerics::replace_generics_new(
                log,
                arena,
                builtin_types,
                level,
                scope,
                Vec::new(),
                Vec::new(),
            ),
        }
    }

    fn install_substitution_vtable(&mut self) {
        let owner = self as *mut Instantiation as *mut c_void;
        self.base.base.vtable = SubstitutionVtable {
            owner,
            is_dirty_ty: Some(instantiation_is_dirty_ty),
            is_dirty_tp: Some(instantiation_is_dirty_tp),
            clean_ty: Some(instantiation_clean_ty),
            clean_tp: Some(instantiation_clean_tp),
            found_dirty_ty: Some(instantiation_found_dirty_ty),
            found_dirty_tp: Some(instantiation_found_dirty_tp),
            ignore_children_ty: Some(instantiation_ignore_children_ty),
            ignore_children_tp: Some(instantiation_ignore_children_tp),
            ignore_children_visit_ty: Some(instantiation_ignore_children_ty),
            ignore_children_visit_tp: Some(instantiation_ignore_children_tp),
        };
    }

    pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
        self.install_substitution_vtable();
        self.base.substitute_type_id(ty)
    }

    pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
        self.install_substitution_vtable();
        self.base.substitute_type_pack_id(tp)
    }
}
