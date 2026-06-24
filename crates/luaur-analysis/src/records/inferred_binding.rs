use crate::records::scope::Scope;
use crate::records::type_ids::TypeIds;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct InferredBinding {
    pub scope: *mut Scope,
    pub location: Location,
    pub types: TypeIds,
}
