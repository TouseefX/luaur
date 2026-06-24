use crate::records::type_arena::TypeArena;
use crate::records::type_function::TypeFunction;
use crate::type_aliases::type_id::TypeId;

impl TypeArena {
    pub fn add_type_function_type_function_initializer_list_type_id(
        &mut self,
        function: &TypeFunction,
        types: &[TypeId],
    ) -> TypeId {
        let mut pack_arguments = Vec::new();
        self.add_type_function_type_function_vector_type_id_vector_type_pack_id(
            function,
            types.to_vec(),
            pack_arguments,
        )
    }
}
