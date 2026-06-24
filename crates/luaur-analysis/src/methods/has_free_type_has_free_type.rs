use crate::records::has_free_type::HasFreeType;

pub fn has_free_type_has_free_type() {
    let mut _visitor = HasFreeType {
        base: crate::records::type_once_visitor::TypeOnceVisitor::new(
            "TypeOnceVisitor".to_string(),
            true,
        ),
        result: false,
    };

    _visitor.has_free_type_has_free_type();
}
