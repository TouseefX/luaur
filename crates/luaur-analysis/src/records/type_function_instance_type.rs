use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::records::type_function::TypeFunction;
use crate::records::user_defined_function_data::UserDefinedFunctionData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::ast_name::AstName;

#[derive(Debug, Clone)]
pub struct TypeFunctionInstanceType {
    pub(crate) function: NonNull<TypeFunction>,
    pub(crate) type_arguments: Vec<TypeId>,
    pub(crate) pack_arguments: Vec<TypePackId>,
    pub(crate) user_func_name: Option<AstName>,
    pub(crate) user_func_data: UserDefinedFunctionData,
    pub(crate) state: TypeFunctionInstanceState,
}

impl TypeFunctionInstanceType {
    pub fn new(
        function: NonNull<TypeFunction>,
        type_arguments: Vec<TypeId>,
        pack_arguments: Vec<TypePackId>,
        user_func_name: Option<AstName>,
        user_func_data: UserDefinedFunctionData,
    ) -> Self {
        Self {
            function,
            type_arguments,
            pack_arguments,
            user_func_name,
            user_func_data,
            state: TypeFunctionInstanceState::Unsolved,
        }
    }

    pub fn new_with_args(function: &TypeFunction, type_arguments: Vec<TypeId>) -> Self {
        Self {
            function: NonNull::from(function),
            type_arguments,
            pack_arguments: Vec::new(),
            user_func_name: None,
            user_func_data: UserDefinedFunctionData::new_empty(),
            state: TypeFunctionInstanceState::Unsolved,
        }
    }

    pub fn new_with_pack_args(
        function: &TypeFunction,
        type_arguments: Vec<TypeId>,
        pack_arguments: Vec<TypePackId>,
    ) -> Self {
        Self {
            function: NonNull::from(function),
            type_arguments,
            pack_arguments,
            user_func_name: None,
            user_func_data: UserDefinedFunctionData::new_empty(),
            state: TypeFunctionInstanceState::Unsolved,
        }
    }

    pub fn new_with_non_null(
        function: NonNull<TypeFunction>,
        type_arguments: Vec<TypeId>,
        pack_arguments: Vec<TypePackId>,
    ) -> Self {
        Self {
            function,
            type_arguments,
            pack_arguments,
            user_func_name: None,
            user_func_data: UserDefinedFunctionData::new_empty(),
            state: TypeFunctionInstanceState::Unsolved,
        }
    }
}

impl Drop for TypeFunctionInstanceType {
    fn drop(&mut self) {
        unsafe {
            core::ptr::write(&mut self.type_arguments, Vec::new());
            core::ptr::write(&mut self.pack_arguments, Vec::new());
            core::ptr::write(
                &mut self.user_func_data,
                UserDefinedFunctionData::new_empty(),
            );
        }
    }
}
