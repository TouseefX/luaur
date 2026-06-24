use crate::enums::table_state::TableState;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::clone_public_interface::ClonePublicInterface;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_finder::GenericTypeFinder;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

/// `GenericTypeFinder::visit` overrides are declared as inherent methods on the
/// record (Instantiation.h:84-114). The visitor driver `traverse` needs the
/// `GenericTypeVisitorTrait` surface, so wire the trait to those inherent
/// methods here (the dispatch points `ClonePublicInterface::clean` relies on).
impl GenericTypeVisitorTrait for GenericTypeFinder {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn visit_type_id(&mut self, ty: TypeId) -> bool {
        GenericTypeFinder::visit_type_id(self, ty)
    }

    fn visit_type_pack_id(&mut self, tp: crate::type_aliases::type_pack_id::TypePackId) -> bool {
        GenericTypeFinder::visit_type_pack_id(self, tp)
    }

    fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
        GenericTypeFinder::visit_type_id_function_type(self, ty, ftv)
    }

    fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
        GenericTypeFinder::visit_type_id_table_type(self, ty, ttv)
    }

    fn visit_type_id_generic_type(&mut self, ty: TypeId, gtv: &GenericType) -> bool {
        GenericTypeFinder::visit_type_id_generic_type(self, ty, gtv)
    }

    fn visit_type_pack_id_generic_type_pack(
        &mut self,
        tp: crate::type_aliases::type_pack_id::TypePackId,
        gtp: &crate::records::generic_type_pack::GenericTypePack,
    ) -> bool {
        GenericTypeFinder::visit_type_pack_id_generic_type_pack(self, tp, gtp)
    }

    fn visit_type_id_extern_type(
        &mut self,
        ty: TypeId,
        etv: &crate::records::extern_type::ExternType,
    ) -> bool {
        GenericTypeFinder::visit_type_id_extern_type(self, ty, etv)
    }
}

impl ClonePublicInterface {
    /// `TypeId ClonePublicInterface::clean(TypeId ty)`.
    /// Reference: `Module.cpp:167-208`.
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        let mut result = self.base.clone_type_id(ty);

        let ftv = unsafe { get_mutable_type_id::<FunctionType>(result) };
        if !ftv.is_null() {
            let ftv = unsafe { &mut *ftv };
            if ftv.generics.is_empty() && ftv.generic_packs.is_empty() {
                let mut marker = GenericTypeFinder::generic_type_finder();
                marker.traverse_type_id(result);

                if !marker.found {
                    ftv.has_no_free_or_generic_types = true;
                }
            }

            ftv.level = TypeLevel::new(0, 0);
        } else {
            let ttv = unsafe { get_mutable_type_id::<TableType>(result) };
            if !ttv.is_null() {
                let ttv = unsafe { &mut *ttv };
                ttv.level = TypeLevel::new(0, 0);
                if self.is_new_solver() {
                    ttv.scope = core::ptr::null_mut();
                    ttv.state = TableState::Sealed;
                }
            }
        }

        if self.is_new_solver() {
            if unsafe {
                !get_type_id::<FreeType>(ty).is_null()
                    || !get_type_id::<BlockedType>(ty).is_null()
                    || !get_type_id::<PendingExpansionType>(ty).is_null()
            } {
                self.internal_type_escaped = true;
                result = unsafe { (*self.builtin_types).errorType };
            } else {
                let genericty = unsafe { get_mutable_type_id::<GenericType>(result) };
                if !genericty.is_null() {
                    unsafe { (*genericty).scope = core::ptr::null_mut() };
                }
            }
        }

        result
    }
}
