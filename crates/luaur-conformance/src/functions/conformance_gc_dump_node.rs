use core::ffi::{c_char, c_void};
use std::ffi::CStr;

use crate::records::conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext;
use crate::records::conformance_gc_dump_node::ConformanceGcDumpNode;
use luaur_vm::enums::lua_type::lua_Type;

pub unsafe extern "C" fn conformance_gc_dump_node(
    context: *mut c_void,
    ptr: *mut c_void,
    tt: u8,
    memcat: u8,
    size: usize,
    name: *const c_char,
) {
    let context = &mut *(context as *mut ConformanceGcDumpEnumContext);
    let name = if name.is_null() {
        String::new()
    } else {
        CStr::from_ptr(name).to_string_lossy().into_owned()
    };

    if !name.is_empty() {
        match tt as i32 {
            x if x == lua_Type::LUA_TUSERDATA as i32 => {
                if name != "u42" {
                    context
                        .errors
                        .push(format!("unexpected userdata name: {name}"));
                }
            }
            x if x == lua_Type::LUA_TPROTO as i32 => {
                if !matches!(
                    name.as_str(),
                    "proto unnamed:1 =GCDump" | "proto foo:7 =GCDump" | "proto f:4 =GCDump"
                ) {
                    context
                        .errors
                        .push(format!("unexpected proto name: {name}"));
                }
            }
            x if x == lua_Type::LUA_TFUNCTION as i32 => {
                if !matches!(
                    name.as_str(),
                    "test" | "unnamed:1 =GCDump" | "foo:7 =GCDump" | "f:4 =GCDump"
                ) {
                    context
                        .errors
                        .push(format!("unexpected function name: {name}"));
                }
            }
            x if x == lua_Type::LUA_TTHREAD as i32 => {
                if name != "thread at unnamed:1 =GCDump" {
                    context
                        .errors
                        .push(format!("unexpected thread name: {name}"));
                }
            }
            _ => {}
        }
    } else if tt as i32 == lua_Type::LUA_TSTRING as i32 && size >= 100_000 {
        if context.seen_target_string {
            context
                .errors
                .push("saw more than one target-sized string".to_owned());
        }

        context.seen_target_string = true;

        if size <= 100_000 {
            context.errors.push(format!(
                "target string size did not include overhead: {size}"
            ));
        }
    }

    context.nodes.insert(
        ptr as usize,
        ConformanceGcDumpNode {
            ptr: ptr as usize,
            tag: tt,
            memcat,
            size,
            name,
        },
    );
}
