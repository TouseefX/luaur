use crate::records::type_rehydration_options::TypeRehydrationOptions;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_type::AstType;
use luaur_common::functions::visit_variant_alt_b::visit_mut;

pub fn rehydrate_annotation(
    r#type: TypeId,
    allocator: *mut Allocator,
    options: &TypeRehydrationOptions,
) -> *mut AstType {
    let mut synthetic_names: SyntheticNames = SyntheticNames::new(core::ptr::null_mut());
    let visitor = TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
        allocator,
        &mut synthetic_names as *mut SyntheticNames,
        options,
    );

    unsafe { visit_mut(visitor, &mut (*r#type.cast_mut()).ty) }
}
