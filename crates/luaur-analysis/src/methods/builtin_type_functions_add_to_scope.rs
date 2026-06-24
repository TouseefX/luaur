use crate::enums::polarity::Polarity;
use crate::records::builtin_type_functions::BuiltinTypeFunctions;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_fun::TypeFun;
use crate::records::type_function::TypeFunction;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use core::ptr::NonNull;

impl BuiltinTypeFunctions {
    pub fn add_to_scope(&self, arena: *mut TypeArena, scope: *mut Scope) {
        // closure for unary type function
        let mk_unary = |tf: &TypeFunction| -> TypeFun {
            let t = unsafe {
                (*arena).add_type(GenericType::generic_type_name_polarity(
                    &"T".to_string(),
                    Polarity::Negative,
                ))
            };
            let generic_t = GenericTypeDefinition {
                ty: t,
                defaultValue: None,
            };
            let result_type = unsafe {
                (*arena).add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
                    NonNull::new(tf as *const TypeFunction as *mut TypeFunction).unwrap(),
                    vec![t],
                    vec![],
                ))
            };
            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                vec![generic_t],
                result_type,
                None,
            )
        };

        // closure for binary type function with default (second type defaults to first)
        let mk_binary_with_default = |tf: &TypeFunction| -> TypeFun {
            let t = unsafe {
                (*arena).add_type(GenericType::generic_type_name_polarity(
                    &"T".to_string(),
                    Polarity::Negative,
                ))
            };
            let u = unsafe {
                (*arena).add_type(GenericType::generic_type_name_polarity(
                    &"U".to_string(),
                    Polarity::Negative,
                ))
            };
            let generic_t = GenericTypeDefinition {
                ty: t,
                defaultValue: None,
            };
            let generic_u = GenericTypeDefinition {
                ty: u,
                defaultValue: Some(t),
            };
            let result_type = unsafe {
                (*arena).add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
                    NonNull::new(tf as *const TypeFunction as *mut TypeFunction).unwrap(),
                    vec![t, u],
                    vec![],
                ))
            };
            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                vec![generic_t, generic_u],
                result_type,
                None,
            )
        };

        // closure for binary type function without default
        let mk_binary = |tf: &TypeFunction| -> TypeFun {
            let t = unsafe {
                (*arena).add_type(GenericType::generic_type_name_polarity(
                    &"T".to_string(),
                    Polarity::Negative,
                ))
            };
            let u = unsafe {
                (*arena).add_type(GenericType::generic_type_name_polarity(
                    &"U".to_string(),
                    Polarity::Negative,
                ))
            };
            let generic_t = GenericTypeDefinition {
                ty: t,
                defaultValue: None,
            };
            let generic_u = GenericTypeDefinition {
                ty: u,
                defaultValue: None,
            };
            let result_type = unsafe {
                (*arena).add_type(TypeFunctionInstanceType::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
                    NonNull::new(tf as *const TypeFunction as *mut TypeFunction).unwrap(),
                    vec![t, u],
                    vec![],
                ))
            };
            TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                vec![generic_t, generic_u],
                result_type,
                None,
            )
        };

        unsafe {
            (*scope)
                .exported_type_bindings
                .insert(self.len_func.name.clone(), mk_unary(&self.len_func));
            (*scope)
                .exported_type_bindings
                .insert(self.unm_func.name.clone(), mk_unary(&self.unm_func));

            (*scope).exported_type_bindings.insert(
                self.add_func.name.clone(),
                mk_binary_with_default(&self.add_func),
            );
            (*scope).exported_type_bindings.insert(
                self.sub_func.name.clone(),
                mk_binary_with_default(&self.sub_func),
            );
            (*scope).exported_type_bindings.insert(
                self.mul_func.name.clone(),
                mk_binary_with_default(&self.mul_func),
            );
            (*scope).exported_type_bindings.insert(
                self.div_func.name.clone(),
                mk_binary_with_default(&self.div_func),
            );
            (*scope).exported_type_bindings.insert(
                self.idiv_func.name.clone(),
                mk_binary_with_default(&self.idiv_func),
            );
            (*scope).exported_type_bindings.insert(
                self.pow_func.name.clone(),
                mk_binary_with_default(&self.pow_func),
            );
            (*scope).exported_type_bindings.insert(
                self.mod_func.name.clone(),
                mk_binary_with_default(&self.mod_func),
            );
            (*scope).exported_type_bindings.insert(
                self.concat_func.name.clone(),
                mk_binary_with_default(&self.concat_func),
            );

            (*scope).exported_type_bindings.insert(
                self.lt_func.name.clone(),
                mk_binary_with_default(&self.lt_func),
            );
            (*scope).exported_type_bindings.insert(
                self.le_func.name.clone(),
                mk_binary_with_default(&self.le_func),
            );
            (*scope)
                .exported_type_bindings
                .insert(self.keyof_func.name.clone(), mk_unary(&self.keyof_func));
            (*scope).exported_type_bindings.insert(
                self.rawkeyof_func.name.clone(),
                mk_unary(&self.rawkeyof_func),
            );

            (*scope)
                .exported_type_bindings
                .insert(self.index_func.name.clone(), mk_binary(&self.index_func));
            (*scope)
                .exported_type_bindings
                .insert(self.rawget_func.name.clone(), mk_binary(&self.rawget_func));

            (*scope).exported_type_bindings.insert(
                self.setmetatable_func.name.clone(),
                mk_binary(&self.setmetatable_func),
            );
            (*scope).exported_type_bindings.insert(
                self.getmetatable_func.name.clone(),
                mk_unary(&self.getmetatable_func),
            );
        }
    }
}
