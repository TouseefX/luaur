use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use core::ffi::c_char;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_singleton_bool::AstTypeSingletonBool;
use luaur_ast::records::ast_type_singleton_string::AstTypeSingletonString;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_16(&mut self, stv: &SingletonType) -> *mut AstType {
        unsafe {
            let bs =
                crate::functions::get_singleton_type::get_singleton_type::<BooleanSingleton>(stv);
            if !bs.is_null() {
                let location = Location::default();
                return self
                    .allocator
                    .as_mut()
                    .unwrap()
                    .alloc(AstTypeSingletonBool::new(location, (*bs).value))
                    as *mut AstType;
            }
            let ss =
                crate::functions::get_singleton_type::get_singleton_type::<StringSingleton>(stv);
            if !ss.is_null() {
                let location = Location::default();
                let value = {
                    let ss_ref = &*ss;
                    let s = &ss_ref.value;
                    let data = s.as_ptr() as *mut c_char;
                    let size = s.len();
                    AstArray { data, size }
                };
                return self
                    .allocator
                    .as_mut()
                    .unwrap()
                    .alloc(AstTypeSingletonString::new(location, value))
                    as *mut AstType;
            }
            core::ptr::null_mut()
        }
    }
}
