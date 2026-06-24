use crate::enums::type_kind::TypeKind;
use crate::records::lint_unknown_type::LintUnknownType;
use alloc::string::String;

impl LintUnknownType {
    pub fn get_type_kind(&mut self, name: &str) -> TypeKind {
        match name {
            "nil" | "boolean" | "userdata" | "number" | "string" | "table" | "function"
            | "thread" | "buffer" | "vector" => TypeKind::Kind_Primitive,
            _ => {
                let context = unsafe { &*self.context };
                if context.scope.lookup_type(&String::from(name)).is_some() {
                    TypeKind::Kind_Userdata
                } else {
                    TypeKind::Kind_Unknown
                }
            }
        }
    }
}
