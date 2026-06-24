use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::records::type_function::TypeFunction;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::user_defined_function_data::UserDefinedFunctionData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_ast::records::ast_name::AstName;

impl TypeFunctionInstanceType {
    pub fn type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id_optional_ast_name_user_defined_function_data(
        function: NonNull<TypeFunction>,
        type_arguments: alloc::vec::Vec<TypeId>,
        pack_arguments: alloc::vec::Vec<TypePackId>,
        user_func_name: Option<AstName>,
        user_func_data: UserDefinedFunctionData,
    ) -> Self {
        Self {
            function,
            type_arguments,
            pack_arguments,
            user_func_name,
            user_func_data,
            state: TypeFunctionInstanceState::default(),
        }
    }
}
