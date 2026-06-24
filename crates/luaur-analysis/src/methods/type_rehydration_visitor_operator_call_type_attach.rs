use crate::records::primitive_type::PrimitiveType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    /// C++ `AstType* operator()(const PrimitiveType& ptv)`.
    pub fn operator_call_15(&mut self, ptv: &PrimitiveType) -> *mut AstType {
        let allocator = unsafe { &mut *self.allocator };
        let location = Location::default();

        let name = match ptv.r#type {
            crate::records::primitive_type::Type::NilType => {
                AstName::ast_name_c_char(c"nil".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Boolean => {
                AstName::ast_name_c_char(c"boolean".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Number => {
                AstName::ast_name_c_char(c"number".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Integer => {
                AstName::ast_name_c_char(c"integer".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::String => {
                AstName::ast_name_c_char(c"string".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Thread => {
                AstName::ast_name_c_char(c"thread".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Buffer => {
                AstName::ast_name_c_char(c"buffer".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Function => {
                AstName::ast_name_c_char(c"function".as_ptr() as *const core::ffi::c_char)
            }
            crate::records::primitive_type::Type::Table => {
                AstName::ast_name_c_char(c"table".as_ptr() as *const core::ffi::c_char)
            }
        };

        let result = allocator.alloc(AstTypeReference::new(
            location,
            None,
            name,
            None,
            location,
            false,
            luaur_ast::records::ast_array::AstArray::default(),
        ));

        result as *mut AstType
    }
}
