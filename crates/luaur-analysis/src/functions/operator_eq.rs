use crate::records::identifier::Identifier;

#[allow(non_snake_case)]
pub fn operator_eq(lhs: &Identifier, rhs: &Identifier) -> bool {
    lhs.name() == rhs.name() && lhs.ctx() == rhs.ctx()
}
