use crate::records::type_function::TypeFunction;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::user_defined_function_data::UserDefinedFunctionData;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;

impl TypeFunctionInstanceType {
    pub fn type_function_instance_type_type_function_vector_type_id(
        function: &TypeFunction,
        type_arguments: alloc::vec::Vec<TypeId>,
    ) -> Self {
        Self::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id_optional_ast_name_user_defined_function_data(
            NonNull::from(function),
            type_arguments,
            alloc::vec::Vec::new(),
            None,
            UserDefinedFunctionData::new_empty(),
        )
    }
}
