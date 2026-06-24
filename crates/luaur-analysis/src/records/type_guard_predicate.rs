use crate::type_aliases::l_value::LValue;
use luaur_ast::records::location::Location;
use luaur_common::records::variant::Variant2;

#[derive(Debug, Clone)]
pub struct TypeGuardPredicate {
    pub lvalue: LValue,
    pub location: Location,
    pub kind: String,
    pub is_typeof: bool,
}
