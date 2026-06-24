use crate::enums::type_type_function_runtime::Type;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::records::type_function_any_type::TypeFunctionAnyType;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use alloc::string::String;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

pub fn get_tag(L: *mut lua_State, ty: TypeFunctionTypeId) -> String {
    unsafe {
        if let Some(n) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if n.r#type == Type::NilType {
                return "nil".to_string();
            }
        }

        if let Some(b) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if b.r#type == Type::Boolean {
                return "boolean".to_string();
            }
        }

        if let Some(n) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if n.r#type == Type::Number {
                return "number".to_string();
            }
        }

        if let Some(n) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if luaur_common::FFlag::LuauIntegerType2.get() && n.r#type == Type::Integer {
                return "integer".to_string();
            }
        }

        if let Some(s) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if s.r#type == Type::String {
                return "string".to_string();
            }
        }

        if let Some(s) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if s.r#type == Type::Thread {
                return "thread".to_string();
            }
        }

        if let Some(s) = get_type_function_type_id::<TypeFunctionPrimitiveType>(ty).as_ref() {
            if s.r#type == Type::Buffer {
                return "buffer".to_string();
            }
        }

        if !get_type_function_type_id::<TypeFunctionUnknownType>(ty).is_null() {
            return "unknown".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionNeverType>(ty).is_null() {
            return "never".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionAnyType>(ty).is_null() {
            return "any".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionSingletonType>(ty).is_null() {
            return "singleton".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionNegationType>(ty).is_null() {
            return "negation".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionUnionType>(ty).is_null() {
            return "union".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionIntersectionType>(ty).is_null() {
            return "intersection".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionTableType>(ty).is_null() {
            return "table".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionFunctionType>(ty).is_null() {
            return "function".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionExternType>(ty).is_null() {
            return "extern".to_string();
        }

        if !get_type_function_type_id::<TypeFunctionGenericType>(ty).is_null() {
            return "generic".to_string();
        }

        LUAU_ASSERT!(false);
        lua_l_error_l(
            L as *mut luaur_vm::records::lua_state::lua_State,
            c"%s".as_ptr(),
            core::format_args!("VM encountered unexpected type variant when determining tag"),
        );
        String::new()
    }
}
