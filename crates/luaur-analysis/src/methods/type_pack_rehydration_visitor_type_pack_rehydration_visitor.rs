use luaur_ast::records::allocator::Allocator;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;

impl TypePackRehydrationVisitor {
    pub fn type_pack_rehydration_visitor_type_pack_rehydration_visitor(
        allocator: *mut Allocator,
        synthetic_names: *mut SyntheticNames,
        type_visitor: *mut TypeRehydrationVisitor,
    ) -> Self {
        LUAU_ASSERT!(allocator != core::ptr::null_mut());
        LUAU_ASSERT!(synthetic_names != core::ptr::null_mut());
        LUAU_ASSERT!(type_visitor != core::ptr::null_mut());

        Self {
            allocator,
            synthetic_names,
            type_visitor,
        }
    }
}
