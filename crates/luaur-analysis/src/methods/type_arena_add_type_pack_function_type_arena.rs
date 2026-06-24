use crate::records::type_arena::TypeArena;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack_function::TypePackFunction;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeArena {
    pub fn add_type_pack_function_type_pack_function_initializer_list_type_id(
        &mut self,
        function: &TypePackFunction,
        types: &[TypeId],
    ) -> TypePackId {
        let type_function_instance_type_pack = TypeFunctionInstanceTypePack {
            function,
            typeArguments: types.to_vec(),
            packArguments: Vec::new(),
        };
        self.add_type_pack_function_type_pack_function_vector_type_id_vector_type_pack_id(
            function,
            types.to_vec(),
            Vec::<TypePackId>::new(),
        )
    }
}
