use crate::records::quantifier::Quantifier;
use crate::records::type_level::TypeLevel;

impl Quantifier {
    pub fn quantifier(level: TypeLevel) -> Self {
        Quantifier {
            base: crate::records::type_once_visitor::TypeOnceVisitor::new(
                "Quantifier".to_string(),
                false,
            ),
            level,
            generics: alloc::vec::Vec::new(),
            generic_packs: alloc::vec::Vec::new(),
            scope: core::ptr::null_mut(),
            seen_generic_type: false,
            seen_mutable_type: false,
        }
    }
}
