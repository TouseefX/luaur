use crate::records::cell::Cell;
use crate::records::phi::Phi;
use crate::records::symbol::Symbol;
use luaur_ast::records::location::Location;

use crate::type_aliases::variant::Variant as VariantAlias;

#[derive(Debug, Clone)]
pub struct Def {
    pub v: VariantAlias,
    pub name: Symbol,
    pub location: Location,
}
