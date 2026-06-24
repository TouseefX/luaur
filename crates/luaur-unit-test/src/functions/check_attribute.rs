use luaur_ast::records::ast_attr::{AstAttr, AstAttrType};
use luaur_ast::records::location::Location;

pub fn check_attribute(attr: &AstAttr, r#type: AstAttrType, location: Location) {
    assert_eq!(attr.r#type, r#type);
    assert_eq!(attr.base.location, location);
}
