use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::functions::escape::escape;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeStringifier {
    pub fn operator_call_18(&mut self, _ty: TypeId, stv: &SingletonType) {
        unsafe {
            match stv.variant {
                crate::type_aliases::singleton_variant::SingletonVariant::V0(ref bs) => {
                    if bs.value {
                        (*self.state).emit_string("true");
                    } else {
                        (*self.state).emit_string("false");
                    }
                }
                crate::type_aliases::singleton_variant::SingletonVariant::V1(ref ss) => {
                    (*self.state).emit_string("\"");
                    let escaped = escape(&ss.value, false);
                    (*self.state).emit_string(&escaped);
                    (*self.state).emit_string("\"");
                }
                _ => {
                    LUAU_ASSERT!(false);
                    panic!("Unknown singleton type");
                }
            }
        }
    }
}
