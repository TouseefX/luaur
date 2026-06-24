use crate::functions::to_string_to_string::to_string_type_pack_id_to_string_options_mut;
use crate::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::functions::wrong_number_of_args_string::wrong_number_of_args_string;
use crate::records::error_converter::ErrorConverter;
use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;
use crate::records::to_string_options::ToStringOptions;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_29(&self, e: &IncorrectGenericParameterCount) -> String {
        let mut name = e.name.clone();
        let mut opts = ToStringOptions::default();

        if !e.type_fun.type_params.is_empty() || !e.type_fun.type_pack_params.is_empty() {
            name.push('<');
            let mut first = true;

            for param in &e.type_fun.type_params {
                if first {
                    first = false;
                } else {
                    name.push_str(", ");
                }

                let ty_str = to_string_type_id_to_string_options_mut(param.ty, opts.clone());
                name.push_str(&ty_str);
            }

            for param in &e.type_fun.type_pack_params {
                if first {
                    first = false;
                } else {
                    name.push_str(", ");
                }

                let tp_str = to_string_type_pack_id_to_string_options_mut(param.tp, opts.clone());
                name.push_str(&tp_str);
            }

            name.push('>');
        }

        if e.type_fun.type_params.len() != e.actual_parameters {
            let is_variadic = !e.type_fun.type_pack_params.is_empty();
            return format!(
                "Generic type '{}' {}",
                name,
                wrong_number_of_args_string(
                    e.type_fun.type_params.len(),
                    None,
                    e.actual_parameters,
                    c"type".as_ptr(),
                    is_variadic
                )
            );
        }

        format!(
            "Generic type '{}' {}",
            name,
            wrong_number_of_args_string(
                e.type_fun.type_pack_params.len(),
                None,
                e.actual_pack_parameters,
                c"type pack".as_ptr(),
                false,
            )
        )
    }
}
