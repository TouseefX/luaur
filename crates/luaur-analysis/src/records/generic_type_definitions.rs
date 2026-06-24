use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::generic_type_pack_definition::GenericTypePackDefinition;

#[derive(Debug, Clone, Default)]
pub struct GenericTypeDefinitions {
    pub generic_types: alloc::vec::Vec<GenericTypeDefinition>,
    pub generic_packs: alloc::vec::Vec<GenericTypePackDefinition>,
}
