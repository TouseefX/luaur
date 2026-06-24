use crate::functions::get_bytecode_type_name::get_bytecode_type_name;
use crate::functions::try_find_local_name::try_find_local_name;
use crate::functions::try_find_upvalue_name::try_find_upvalue_name;
use crate::records::bytecode_reg_type_info::LBC_TYPE_ANY;
use crate::records::ir_function::IrFunction;
use luaur_vm::records::proto::Proto;

pub const LBC_TYPE_OPTIONAL_BIT: u8 = 1 << 7;

pub unsafe fn log_function_types(
    build: &mut dyn crate::traits::LogAppend,
    function: &IrFunction,
    userdata_types: *const *const core::ffi::c_char,
) {
    let type_info = &function.bc_type_info;

    for i in 0..type_info.argument_types.len() {
        let ty = unsafe { *type_info.argument_types.as_ptr().add(i) };

        let r#type = get_bytecode_type_name(ty, userdata_types);
        let optional = if (ty & LBC_TYPE_OPTIONAL_BIT) != 0 {
            "?"
        } else {
            ""
        };

        if ty != LBC_TYPE_ANY {
            let type_str = core::ffi::CStr::from_ptr(r#type).to_string_lossy();
            let name = try_find_local_name(function.proto as *mut Proto, i as i32, 0);

            if !name.is_null() {
                let name_str = core::ffi::CStr::from_ptr(name).to_string_lossy();
                build.log_append(format_args!(
                    "; R{}: {}{} [argument '{}']\n",
                    i, type_str, optional, name_str
                ));
            } else {
                build.log_append(format_args!(
                    "; R{}: {}{} [argument]\n",
                    i, type_str, optional
                ));
            }
        }
    }

    for i in 0..type_info.upvalue_types.len() {
        let ty = unsafe { *type_info.upvalue_types.as_ptr().add(i) };

        let r#type = get_bytecode_type_name(ty, userdata_types);
        let optional = if (ty & LBC_TYPE_OPTIONAL_BIT) != 0 {
            "?"
        } else {
            ""
        };

        if ty != LBC_TYPE_ANY {
            let type_str = core::ffi::CStr::from_ptr(r#type).to_string_lossy();
            let name = try_find_upvalue_name(function.proto as *const Proto, i as i32);

            if !name.is_null() {
                let name_str = core::ffi::CStr::from_ptr(name).to_string_lossy();
                build.log_append(format_args!(
                    "; U{}: {}{} ['{}']\n",
                    i, type_str, optional, name_str
                ));
            } else {
                build.log_append(format_args!("; U{}: {}{}\n", i, type_str, optional));
            }
        }
    }

    for i in 0..type_info.reg_types.len() {
        let el = unsafe { &*type_info.reg_types.as_ptr().add(i) };

        let r#type = get_bytecode_type_name(el.r#type, userdata_types);
        let optional = if (el.r#type & LBC_TYPE_OPTIONAL_BIT) != 0 {
            "?"
        } else {
            ""
        };

        let type_str = core::ffi::CStr::from_ptr(r#type).to_string_lossy();

        // Using last active position as the PC because 'startpc' for type info is before local is initialized
        let name = try_find_local_name(function.proto as *mut Proto, el.reg as i32, el.endpc - 1);

        if !name.is_null() {
            let name_str = core::ffi::CStr::from_ptr(name).to_string_lossy();
            build.log_append(format_args!(
                "; R{}: {}{} from {} to {} [local '{}']\n",
                el.reg, type_str, optional, el.startpc, el.endpc, name_str
            ));
        } else {
            build.log_append(format_args!(
                "; R{}: {}{} from {} to {}\n",
                el.reg, type_str, optional, el.startpc, el.endpc
            ));
        }
    }
}
