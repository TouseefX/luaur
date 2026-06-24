use alloc::string::String;
use core::ffi::c_int;
use luaur_common::functions::format::format;
use luaur_common::records::variant::Variant5;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_isstring::lua_isstring;
use luaur_vm::functions::lua_typename::lua_typename;
use luaur_vm::macros::lua_tostring::lua_tostring;

use crate::records::runtime_error::RuntimeError;
use crate::records::type_function_error::TypeFunctionError;
use crate::type_aliases::lua_state::lua_State;
use luaur_ast::records::location::Location;

pub fn check_result_for_error(
    L: *mut lua_State,
    type_function_name: &str,
    lua_result: c_int,
) -> Option<TypeFunctionError> {
    match lua_result {
        0 => None, // LUA_OK
        1 | 3 => Some(
            TypeFunctionError::type_function_error_location_type_function_error_data(
                Location::new(Default::default(), Default::default()),
                Variant5::V2(RuntimeError::new(format(format_args!(
                    "'{}' type function errored: unexpected yield or break",
                    type_function_name
                )))),
            ),
        ), // LUA_YIELD, LUA_BREAK
        _ => {
            if unsafe { lua_gettop(L as *mut luaur_vm::records::lua_state::lua_State) } == 0 {
                Some(
                    TypeFunctionError::type_function_error_location_type_function_error_data(
                        Location::new(Default::default(), Default::default()),
                        Variant5::V2(RuntimeError::new(format(format_args!(
                            "'{}' type function errored unexpectedly",
                            type_function_name
                        )))),
                    ),
                )
            } else if unsafe { lua_isstring(L as *mut luaur_vm::records::lua_state::lua_State, -1) }
                != 0
            {
                let err_str =
                    unsafe { lua_tostring!(L as *mut luaur_vm::records::lua_state::lua_State, -1) };
                let err_str = unsafe { core::ffi::CStr::from_ptr(err_str).to_string_lossy() };
                Some(
                    TypeFunctionError::type_function_error_location_type_function_error_data(
                        Location::new(Default::default(), Default::default()),
                        Variant5::V2(RuntimeError::new(format(format_args!(
                            "'{}' type function errored at runtime: {}",
                            type_function_name, err_str
                        )))),
                    ),
                )
            } else {
                let err_type =
                    unsafe { lua_typename(L as *mut luaur_vm::records::lua_state::lua_State, -1) };
                let err_type = unsafe { core::ffi::CStr::from_ptr(err_type).to_string_lossy() };
                Some(
                    TypeFunctionError::type_function_error_location_type_function_error_data(
                        Location::new(Default::default(), Default::default()),
                        Variant5::V2(RuntimeError::new(format(format_args!(
                            "'{}' type function errored at runtime: raised an error of type {}",
                            type_function_name, err_type
                        )))),
                    ),
                )
            }
        }
    }
}
