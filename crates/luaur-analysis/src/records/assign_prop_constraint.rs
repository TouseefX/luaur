use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignPropConstraint {
    pub(crate) lhs_type: TypeId,
    pub(crate) prop_name: String,
    pub(crate) rhs_type: TypeId,
    pub(crate) prop_location: Option<Location>,
    pub(crate) prop_type: TypeId,
    pub(crate) decrement_prop_count: bool,
}

#[allow(non_snake_case)]
impl AssignPropConstraint {
    pub fn lhsType(&self) -> TypeId {
        self.lhs_type
    }

    pub fn propName(&self) -> &str {
        &self.prop_name
    }

    pub fn rhsType(&self) -> TypeId {
        self.rhs_type
    }

    pub fn propLocation(&self) -> &Option<Location> {
        &self.prop_location
    }

    pub fn propType(&self) -> TypeId {
        self.prop_type
    }

    pub fn decrementPropCount(&self) -> bool {
        self.decrement_prop_count
    }
}
