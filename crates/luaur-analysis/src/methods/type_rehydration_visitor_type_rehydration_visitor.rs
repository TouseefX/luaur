use crate::records::type_rehydration_options::TypeRehydrationOptions;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;
use luaur_ast::records::allocator::Allocator;
use std::collections::BTreeMap;

impl TypeRehydrationVisitor {
    #[inline]
    pub fn type_rehydration_visitor_type_rehydration_visitor(
        alloc: *mut Allocator,
        synthetic_names: *mut SyntheticNames,
        options: &TypeRehydrationOptions,
    ) -> Self {
        Self {
            seen: BTreeMap::new(),
            count: 0,
            allocator: alloc,
            synthetic_names,
            options: options.clone(),
        }
    }
}
