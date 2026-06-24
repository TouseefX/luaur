use crate::enums::polarity::Polarity;
use crate::functions::freeze::freeze;
use crate::functions::unfreeze::unfreeze;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::global_types::GlobalTypes;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::scope::Scope;
use crate::records::type_fun::TypeFun;
use alloc::string::String;
use alloc::sync::Arc;

impl GlobalTypes {
    pub fn register_hidden_test_types(&mut self) {
        unfreeze(&mut self.global_types);

        let t = self
            .global_types
            .add_type(GenericType::generic_type_name_polarity(
                &String::from("T"),
                Polarity::Mixed,
            ));
        let generic_t = GenericTypeDefinition {
            ty: t,
            defaultValue: None,
        };

        let u = self
            .global_types
            .add_type(GenericType::generic_type_name_polarity(
                &String::from("U"),
                Polarity::Mixed,
            ));
        let generic_u = GenericTypeDefinition {
            ty: u,
            defaultValue: None,
        };

        let not_type = self.global_types.add_type(NegationType::new(t));
        let mt_type = self.global_types.add_type(MetatableType {
            table: t,
            metatable: u,
            syntheticName: None,
        });

        let (function_type, extern_type, error_type, table_type) = unsafe {
            let builtins = self.builtin_types.as_ref();
            (
                builtins.functionType,
                builtins.externType,
                builtins.errorType,
                builtins.tableType,
            )
        };

        let scope = Arc::as_ptr(&self.global_scope) as *mut Scope;
        unsafe {
            (*scope).exported_type_bindings.insert(
                String::from("Not"),
                TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                    vec![generic_t],
                    not_type,
                    None,
                ),
            );
            (*scope).exported_type_bindings.insert(
                String::from("Mt"),
                TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                    vec![generic_t, generic_u],
                    mt_type,
                    None,
                ),
            );
            (*scope).exported_type_bindings.insert(
                String::from("fun"),
                TypeFun::type_fun_type_id(function_type),
            );
            (*scope)
                .exported_type_bindings
                .insert(String::from("cls"), TypeFun::type_fun_type_id(extern_type));
            (*scope)
                .exported_type_bindings
                .insert(String::from("err"), TypeFun::type_fun_type_id(error_type));
            (*scope)
                .exported_type_bindings
                .insert(String::from("tbl"), TypeFun::type_fun_type_id(table_type));
        }

        freeze(&mut self.global_types);
    }
}
