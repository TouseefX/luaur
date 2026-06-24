//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:900-911`
//!
//! Dispatch for `void deserializeChildren(TypeFunctionTypePackId tftp, TypePackId tp)`.
use crate::functions::get_mutable_type_function_runtime_alt_f::get_mutable_type_function_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_type_pack_id_type_pack_id(
        &mut self,
        tftp: TypeFunctionTypePackId,
        tp: TypePackId,
    ) {
        unsafe {
            let t_pack1 = get_mutable_type_pack_id::<TypePack>(tp);
            let t_pack2 = get_mutable_type_function_type_pack_id::<TypeFunctionTypePack>(tftp);
            if !t_pack1.is_null() && !t_pack2.is_null() {
                self.deserialize_children_type_function_type_pack_type_pack(t_pack2, t_pack1);
                return;
            }

            let v_pack1 = get_mutable_type_pack_id::<VariadicTypePack>(tp);
            let v_pack2 =
                get_mutable_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tftp);
            if !v_pack1.is_null() && !v_pack2.is_null() {
                self.deserialize_children_type_function_variadic_type_pack_variadic_type_pack(
                    v_pack2, v_pack1,
                );
                return;
            }

            let g_pack1 = get_mutable_type_pack_id::<GenericTypePack>(tp);
            let g_pack2 =
                get_mutable_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tftp);
            if !g_pack1.is_null() && !g_pack2.is_null() {
                self.deserialize_children_type_function_generic_type_pack_generic_type_pack(
                    g_pack2, g_pack1,
                );
                return;
            }

            (*(*self.state).ctx)
                .ice
                .as_ref()
                .ice_string("Deserializing user defined type function arguments: mysterious type is being deserialized");
        }
    }
}
