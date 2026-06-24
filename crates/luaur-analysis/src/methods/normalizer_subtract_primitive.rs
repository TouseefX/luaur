use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn subtract_primitive(&mut self, here: &mut NormalizedType, ty: TypeId) {
        self.consume_fuel();

        let ty_followed = unsafe { crate::functions::follow_type::follow_type_id(ty) };
        let ptv = unsafe {
            crate::functions::get_type_alt_j::get_type_id::<
                crate::records::primitive_type::PrimitiveType,
            >(ty_followed)
        };
        luaur_common::macros::luau_assert::LUAU_ASSERT!(!ptv.is_null());

        let builtin_types = here.builtin_types;
        let ptv_ref = unsafe { &*ptv };
        match ptv_ref.r#type {
            crate::records::primitive_type::Type::NilType => {
                here.nils = unsafe { (*builtin_types).neverType };
            }
            crate::records::primitive_type::Type::Boolean => {
                here.booleans = unsafe { (*builtin_types).neverType };
            }
            crate::records::primitive_type::Type::Number => {
                here.numbers = unsafe { (*builtin_types).neverType };
            }
            crate::records::primitive_type::Type::Integer => {
                here.integers = unsafe { (*builtin_types).neverType };
            }
            crate::records::primitive_type::Type::String => {
                here.strings.reset_to_never();
            }
            crate::records::primitive_type::Type::Thread => {
                here.threads = unsafe { (*builtin_types).neverType };
            }
            crate::records::primitive_type::Type::Buffer => {
                here.buffers = unsafe { (*builtin_types).neverType };
            }
            crate::records::primitive_type::Type::Function => {
                here.functions.reset_to_never();
            }
            crate::records::primitive_type::Type::Table => {
                here.tables.clear();
            }
        }
    }
}
