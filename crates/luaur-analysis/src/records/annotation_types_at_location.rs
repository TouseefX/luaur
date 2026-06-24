use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct AnnotationTypesAtLocation {
    pub location: Location,
    pub resolved_ty: TypeId,
}
