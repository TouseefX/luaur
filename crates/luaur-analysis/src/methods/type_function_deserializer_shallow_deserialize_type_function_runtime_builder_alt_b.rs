use crate::functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_function_kind::TypeFunctionKind;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::format;
use alloc::vec::Vec;

impl TypeFunctionDeserializer {
    pub fn shallow_deserialize_type_function_type_pack_id(
        &mut self,
        tp: TypeFunctionTypePackId,
    ) -> TypePackId {
        if let Some(it) = self.find_type_function_type_pack_id(tp) {
            return it;
        }

        unsafe {
            let ctx = &mut *(*self.state).ctx;
            let arena = ctx.arena.as_ptr();
            let mut target: TypePackId = core::ptr::null();

            if !get_type_function_type_pack_id::<TypeFunctionTypePack>(tp).is_null() {
                target = (*arena).add_type_pack_t(TypePack::new(Vec::new(), None));
            } else if !get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp).is_null()
            {
                target = (*arena).add_type_pack_t(VariadicTypePack::default());
            } else if let Some(g_pack) =
                get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp).as_ref()
            {
                if let Some(mapping) = self
                    .generic_packs
                    .iter()
                    .rev()
                    .find(|el| el.is_named == g_pack.isNamed() && el.name == g_pack.name())
                    .map(|el| el.r#type)
                {
                    target = mapping;
                } else {
                    self.push_runtime_error(format!(
                        "Generic type pack '{}...' is not in a scope of the active generic function",
                        g_pack.name()
                    ));
                    return core::ptr::null();
                }
            } else {
                (*ctx.ice.as_ptr()).ice_string(
                    "Deserializing user defined type function arguments: mysterious type is being deserialized",
                );
                return core::ptr::null();
            }

            *self.packs.get_or_insert(tp) = target;
            self.queue
                .push((TypeFunctionKind::V1(tp), TypeOrPack::V1(target)));
            target
        }
    }
}
