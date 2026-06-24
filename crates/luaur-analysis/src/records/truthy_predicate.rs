use crate::type_aliases::l_value::LValue;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct TruthyPredicate {
    pub lvalue: LValue,
    pub location: Location,
}
