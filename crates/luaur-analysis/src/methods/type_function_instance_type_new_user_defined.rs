use crate::records::type_function::TypeFunction;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::user_defined_function_data::UserDefinedFunctionData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::ast_name::AstName;

impl TypeFunctionInstanceType {
    pub fn new_user_defined(
        function: &TypeFunction,
        type_arguments: Vec<TypeId>,
        pack_arguments: Vec<TypePackId>,
        user_func_name: AstName,
    ) -> Self {
        Self::new(
            core::ptr::NonNull::from(function),
            type_arguments,
            pack_arguments,
            Some(user_func_name),
            UserDefinedFunctionData::new_empty(),
        )
    }
}
