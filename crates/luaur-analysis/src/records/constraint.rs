use crate::records::scope::Scope;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_pack_ids::TypePackIds;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub(crate) scope: *mut Scope,
    pub(crate) location: Location,
    pub(crate) c: ConstraintV,
    pub(crate) deprecated_dependencies: Vec<*mut Constraint>,
}
