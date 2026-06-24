use core::ffi::c_char;
use std::ffi::CString;

use luaur_analysis::records::primitive_type::{PrimitiveType, Type as PrimitiveKind};
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_variant::TypeVariant;
use luaur_common::FFlag::LuauIntegerType2;
use luaur_common::LUAU_ASSERT;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::records::lua_state::lua_State;

unsafe fn push_literal(L: *mut lua_State, value: &'static [u8]) {
    lua_pushstring(L, value.as_ptr() as *const c_char);
}

pub unsafe fn populate_rtti(L: *mut lua_State, ty: TypeId) {
    LUAU_ASSERT!(!ty.is_null());

    match &(*ty).ty {
        TypeVariant::Primitive(PrimitiveType { r#type, .. }) => match r#type {
            PrimitiveKind::Boolean => push_literal(L, b"boolean\0"),
            PrimitiveKind::NilType => push_literal(L, b"nil\0"),
            PrimitiveKind::Number => push_literal(L, b"number\0"),
            PrimitiveKind::Integer => {
                if LuauIntegerType2.get() {
                    push_literal(L, b"integer\0");
                }
            }
            PrimitiveKind::String => push_literal(L, b"string\0"),
            PrimitiveKind::Thread => push_literal(L, b"thread\0"),
            PrimitiveKind::Buffer => push_literal(L, b"buffer\0"),
            _ => LUAU_ASSERT!(false, "Unknown primitive type"),
        },
        TypeVariant::Table(table) => {
            lua_newtable(L);

            for (name, prop) in &table.props {
                if let Some(read_ty) = prop.read_ty {
                    populate_rtti(L, read_ty);
                } else if let Some(write_ty) = prop.write_ty {
                    populate_rtti(L, write_ty);
                } else {
                    continue;
                }

                let field = CString::new(name.as_str()).expect("type property name contains nul");
                lua_setfield(L, -2, field.as_ptr());
            }
        }
        TypeVariant::Function(_) => push_literal(L, b"function\0"),
        TypeVariant::Any(_) => push_literal(L, b"any\0"),
        TypeVariant::Intersection(intersection) => {
            for part in &intersection.parts {
                LUAU_ASSERT!(matches!((*(*part)).ty, TypeVariant::Function(_)));
            }

            push_literal(L, b"function\0");
        }
        TypeVariant::Extern(extern_ty) => {
            let name =
                CString::new(extern_ty.name.as_str()).expect("extern type name contains nul");
            lua_pushstring(L, name.as_ptr());
        }
        _ => LUAU_ASSERT!(false, "Unknown type"),
    }
}
