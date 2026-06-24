use crate::records::type_arena::TypeArena;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack_function::TypePackFunction;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeArena {
    pub fn add_type_pack_function_type_pack_function_vector_type_id_vector_type_pack_id(
        &mut self,
        function: &TypePackFunction,
        type_arguments: alloc::vec::Vec<TypeId>,
        pack_arguments: alloc::vec::Vec<TypePackId>,
    ) -> TypePackId {
        let type_function_instance_type_pack = TypeFunctionInstanceTypePack {
            function,
            typeArguments: type_arguments,
            packArguments: pack_arguments,
        };
        self.add_type_pack_t(type_function_instance_type_pack)
    }
}
