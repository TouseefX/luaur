use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeFun {
    pub fn type_fun_vector_generic_type_definition_type_id_optional_location(
        type_params: Vec<GenericTypeDefinition>,
        r#type: TypeId,
        definition_location: Option<Location>,
    ) -> Self {
        Self {
            type_params,
            type_pack_params: Vec::new(),
            r#type,
            definition_location,
        }
    }
}
