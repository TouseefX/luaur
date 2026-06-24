use crate::type_aliases::l_value::LValue;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsAPredicate {
    pub lvalue: LValue,
    pub location: Location,
    pub ty: TypeId,
}
