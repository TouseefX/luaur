use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl FunctionType {
    pub fn new_with_generics(
        generics: Vec<TypeId>,
        generic_packs: Vec<TypePackId>,
        arg_types: TypePackId,
        ret_types: TypePackId,
        defn: Option<FunctionDefinition>,
        has_self: bool,
    ) -> Self {
        let mut result = Self::function_type_new(arg_types, ret_types, defn, has_self);
        result.generics = generics;
        result.generic_packs = generic_packs;
        result
    }
}
