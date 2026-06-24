use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl FunctionType {
    pub fn function_type_vector_type_id_vector_type_pack_id_type_pack_id_type_pack_id_optional_function_definition_bool(
        &mut self,
        generics: alloc::vec::Vec<TypeId>,
        generic_packs: alloc::vec::Vec<TypePackId>,
        arg_types: TypePackId,
        ret_types: TypePackId,
        defn: Option<FunctionDefinition>,
        has_self: bool,
    ) {
        let level = TypeLevel::default();
        self.function_type_type_level_type_pack_id_type_pack_id_optional_function_definition_bool(
            level, arg_types, ret_types, defn, has_self,
        );
        self.generics = generics;
        self.generic_packs = generic_packs;
    }
}
