use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::error_converter::ErrorConverter;
use crate::records::function_type::FunctionType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::union_type::UnionType;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_13(&self, e: &CannotCallNonFunction) -> String {
        let t = unsafe { follow_type_id(e.ty) };

        let union_ty_ptr = unsafe { get_type_id::<UnionType>(t) };
        if !union_ty_ptr.is_null() {
            let union_ty = unsafe { &*union_ty_ptr };
            let mut err = String::from("Cannot call a value of the union type:");

            for option in &union_ty.options {
                let option = unsafe { follow_type_id(*option) };

                if !unsafe { get_type_id::<FunctionType>(option) }.is_null()
                    || self.find_call_metamethod(option).is_some()
                {
                    err.push_str("\n  | ");
                    err.push_str(&to_string_type_id(option));
                    continue;
                }

                {
                    return format!(
                        "Cannot call a value of type {} in union:\n  {}",
                        to_string_type_id(option),
                        to_string_type_id(e.ty)
                    );
                }
            }

            err.push_str(
                "\nWe are unable to determine the appropriate result type for such a call.",
            );
            return err;
        }

        let primitive_ty_ptr = unsafe { get_type_id::<PrimitiveType>(t) };
        if !primitive_ty_ptr.is_null() {
            let primitive_ty = unsafe { &*primitive_ty_ptr };
            if primitive_ty.r#type == PrimitiveType::Function {
                return format!(
                    "The type {} is not precise enough for us to determine the appropriate result type of this call.",
                    to_string_type_id(e.ty)
                );
            }
        }

        format!("Cannot call a value of type {}", to_string_type_id(e.ty))
    }
}
