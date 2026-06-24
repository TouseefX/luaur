use crate::functions::get_mutable_type::getMutable;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::quantifier::Quantifier;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ffi::c_void;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

// C++ `struct Quantifier final : TypeOnceVisitor` (Quantify.cpp:15). The
// virtual `visit(...)` overrides live as the `GenericTypeVisitorTrait` impl
// (the `IndexCollector`/`FindCyclicTypes` precedents) so `traverse` dispatches
// into them; the bodies delegate to the inherent methods declared on the
// sibling `quantifier_visit_quantify*` files.
impl GenericTypeVisitorTrait for Quantifier {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
        Quantifier::visit_type_id_free_type(self, ty, ftv)
    }

    fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
        Quantifier::visit_type_id_table_type(self, ty, ttv)
    }

    fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
        Quantifier::visit_type_pack_id_free_type_pack(self, tp, ftp)
    }
}

pub fn quantify(ty: TypeId, level: TypeLevel) {
    let mut q = Quantifier::quantifier(level);
    q.traverse_type_id(ty);

    let ftv = unsafe { getMutable::<FunctionType>(ty) };
    LUAU_ASSERT!(!ftv.is_null());
    unsafe {
        (*ftv).generics.extend(q.generics.iter().copied());
        (*ftv).generic_packs.extend(q.generic_packs.iter().copied());
    }
}
